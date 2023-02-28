use crate::{entity::transaction, feed::Feed};
use async_trait::async_trait;
use oura::{
    filters::selection::{self, Predicate},
    mapper,
    model::{BlockRecord, EventData},
    pipelining::{FilterProvider, SourceProvider, StageReceiver},
    sources::{n2n, AddressArg, BearerKind, IntersectArg, MagicArg, PointArg},
    utils::{ChainWellKnownInfo, Utils, WithUtils},
};
use pallas_addresses::ShelleyAddress;
use sea_orm::{entity::*, query::*, ActiveModelTrait, DatabaseConnection, DbBackend, Set, Unset};
use std::{
    collections::{BTreeMap, BTreeSet},
    f32::consts::E,
    str::FromStr,
    sync::Arc,
    thread::JoinHandle,
};

pub fn oura_bootstrap(
    hash: String,
    slot: u64,
    socket: String,
) -> (JoinHandle<()>, JoinHandle<()>, StageReceiver) {
    let magic = MagicArg::from_str("mainnet").unwrap();

    let well_known = ChainWellKnownInfo::try_from_magic(*magic).unwrap();

    let utils = Arc::new(Utils::new(well_known));

    let mapper = mapper::Config {
        include_block_details: true,
        include_transaction_details: true,
        include_block_cbor: true,
        ..Default::default()
    };

    let intersect = Some(IntersectArg::Point(PointArg(slot, hash)));

    #[allow(deprecated)]
    let source_config = n2n::Config {
        address: if socket.contains(':') {
            AddressArg(BearerKind::Tcp, socket)
        } else {
            AddressArg(BearerKind::Unix, socket)
        },
        magic: Some(magic),
        well_known: None,
        mapper,
        since: None,
        min_depth: 0,
        intersect,
        retry_policy: None,
        finalize: None,
    };

    let source_setup = WithUtils::new(source_config, utils);

    let check = Predicate::VariantIn(vec![String::from("Block")]);

    let filter_setup = selection::Config { check };

    tracing::info!("{}", "Attempting to connect to node...");

    let (source_handle, source_rx) = source_setup.bootstrap().unwrap();

    tracing::info!("{}", "Connection to node established");

    let (filter_handle, filter_rx) = filter_setup.bootstrap(source_rx).unwrap();

    (source_handle, filter_handle, filter_rx)
}

/// Convert cardano address to bytes
fn address_to_bytes(address: &String) -> Vec<u8> {
    if let Ok(address) = pallas_addresses::Address::from_bech32(address) {
        address.to_vec()
    } else if let Ok(bytes) = hex::decode(address) {
        bytes
    } else {
        address.as_bytes().to_vec()
    }
}

#[async_trait]
impl super::Feed for shared::Cardano {
    async fn run(
        &mut self,
        db: DatabaseConnection,
        mut receiver: tokio::sync::mpsc::Receiver<super::FeedCommand>,
        chain_id: i32,
    ) {
        tracing::info!("Background job started: Cardano");
        if let Ok(Some(mut chain)) = crate::entity::chain::Entity::find_by_id(chain_id.clone())
            .one(&db)
            .await
        {
            // let run oura - it is not async :(
            let (oura_sender, mut oura_receiver) = tokio::sync::mpsc::channel::<BlockRecord>(10);
            tokio::task::spawn_blocking({
                let block_hash = self.block_hash.clone();
                let slot = self.slot.clone();
                let address = self.address.clone();
                move || {
                    let (skip1, skip2, oura) = oura_bootstrap(block_hash, slot, address.clone());

                    loop {
                        if let Ok(event) = oura.recv() {
                            match event.data {
                                EventData::Block(block) => {
                                    while let Err(_) = oura_sender.try_send(block.clone()) {
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                    }
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                }
            });

            loop {
                // Skip address action
                tracing::info!("Cardano loop");
                while let Ok(_) = receiver.try_recv() {}

                if let Some(block) = oura_receiver.recv().await {
                    if let Some(transactions) = block.transactions {
                        tracing::info!("Oura recv block: {:?}", block.hash);
                        let address_list = transactions
                            .iter()
                            .map(|t| {
                                t.outputs
                                    .iter()
                                    .flatten()
                                    .map(|o| address_to_bytes(&o.address))
                            })
                            .flatten()
                            .collect::<Vec<Vec<u8>>>();

                        // Store all known addres to database
                        self.add_addresses(&db, chain_id, &address_list).await;
                        let output_address_map =
                            self.map_address(&db, chain_id, &address_list).await;

                        // Map input addresses
                        let mut input_address_map: BTreeMap<(Vec<u8>, i64), i64> = BTreeMap::new();
                        let statement = Statement::from_sql_and_values(
                            DbBackend::Postgres,
                            r#"
                                --- Magic query to transalte hash/index to address id

                                WITH inputs as (SELECT * FROM unnest($1, $2) as x(tx_hash, "index"))

                                SELECT I.tx_hash, index, T."to"[I."index"+1] as address_id FROM inputs I LEFT JOIN transaction T
                                    ON I.tx_hash = T.hash
                            "#,
                            vec![
                                // Get TX hashes from input
                                transactions
                                    .iter()
                                    .map(|t| {
                                        t.inputs
                                            .iter()
                                            .flatten()
                                            .map(|i| hex::decode(&i.tx_id).unwrap())
                                    })
                                    .flatten()
                                    .collect::<Vec<Vec<u8>>>()
                                    .into(),
                                // Get indexes fron inputs
                                transactions
                                    .iter()
                                    .map(|t| t.inputs.iter().flatten().map(|i| i.index as i64))
                                    .flatten()
                                    .collect::<Vec<i64>>()
                                    .into(),
                            ],
                        );
                        if let Ok(result) = db.query_all(statement).await {
                            for row in result {
                                input_address_map.insert(
                                    (
                                        row.try_get("", "tx_hash").unwrap(),
                                        row.try_get("", "index").unwrap(),
                                    ),
                                    row.try_get("", "address_id").unwrap_or(0),
                                );
                            }
                        }

                        // Check if transaction exists
                        if let Ok(query) = crate::entity::transaction::Entity::find()
                            .select_only()
                            .column(crate::entity::transaction::Column::Hash)
                            .filter(
                                crate::entity::transaction::Column::Hash.is_in(
                                    transactions
                                        .iter()
                                        .map(|t| hex::decode(&t.hash).unwrap())
                                        .collect::<Vec<Vec<u8>>>(),
                                ),
                            )
                            .all(&db)
                            .await
                        {
                            let existing_transaction: BTreeSet<Vec<u8>> =
                                BTreeSet::from_iter(query.iter().map(|t| t.hash.clone()));

                            // Transactions to add
                            let transaction_to_insert = transactions
                                .iter()
                                .filter(|t| {
                                    existing_transaction.contains(&hex::decode(&t.hash).unwrap())
                                })
                                .map(|t| crate::entity::transaction::ActiveModel {
                                    chain: Set(chain_id.clone()),
                                    hash: Set(hex::decode(&t.hash).unwrap()),
                                    from: Set(t
                                        .inputs
                                        .iter()
                                        .flatten()
                                        .map(|i| {
                                            input_address_map.get(&(
                                                hex::decode(&i.tx_id).unwrap(),
                                                i.index as i64,
                                            ))
                                        })
                                        .filter(|i| i.is_some() && i.unwrap().gt(&0))
                                        .map(|i| i.unwrap().clone())
                                        .collect()),
                                    to: Set(t
                                        .outputs
                                        .iter()
                                        .flatten()
                                        .filter(|a| {
                                            output_address_map
                                                .contains_key(&address_to_bytes(&a.address))
                                        })
                                        .map(|a| {
                                            output_address_map
                                                .get(&address_to_bytes(&a.address))
                                                .unwrap()
                                                .clone()
                                        })
                                        .collect()),
                                    ..Default::default()
                                })
                                .collect::<Vec<crate::entity::transaction::ActiveModel>>();

                            // If there is no transactions, skip
                            if transaction_to_insert.len() > 0 {
                                if let Err(err) = crate::entity::transaction::Entity::insert_many(
                                    transaction_to_insert,
                                )
                                .exec(&db)
                                .await
                                {
                                    tracing::error!("{}", err.to_string());
                                }
                            }
                        }
                    }

                    let mut params: shared::ChainParam =
                        serde_json::from_value(chain.params.clone()).unwrap();

                    if let shared::ChainParam::Cardano(cardano) = &mut params {
                        cardano.block_hash = block.hash.clone();
                        cardano.slot = block.slot.clone();
                    }
                    let mut chain_change: crate::entity::chain::ActiveModel = chain.clone().into();
                    chain_change.params = Set(serde_json::to_value(params).unwrap());

                    if let Ok(new_chain) = chain_change.update(&db).await {
                        chain = new_chain;
                    }
                }
            }
        }
    }
}
