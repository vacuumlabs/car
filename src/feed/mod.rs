use async_trait::async_trait;
use sea_orm::{
    entity::*, query::*, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, DeriveColumn,
    EntityTrait, EnumIter, QueryFilter, QuerySelect, Selector, Set, Statement,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};
use tokio::sync::mpsc::Receiver;

mod anyscan;

#[derive(Clone, Debug)]
pub enum FeedCommand {
    Address(i64),
    Stop,
}

type AddressList = Vec<Vec<u8>>;
type TransactionList = Vec<(Vec<u8>, Option<u128>, Vec<Vec<u8>>, Vec<Vec<u8>>)>;

#[async_trait]
pub trait Feed {
    async fn run(
        &mut self,
        db: DatabaseConnection,
        mut receiver: Receiver<FeedCommand>,
        chain_id: i32,
    ) {
        tracing::info!("Background job started");

        loop {
            let start = tokio::time::Instant::now();

            if let Ok(result) = crate::entity::chain::Entity::find_by_id(chain_id.clone())
                .one(&db)
                .await
            {
                if let Some(mut chain) = result {
                    if let Ok(msg) = receiver.try_recv() {
                        match msg {
                            FeedCommand::Address(a) => {
                                tracing::info!("PROCESSING ADDRESSESSS");
                                self.process_address(&db, a).await
                            }
                            FeedCommand::Stop => break,
                        }
                    } else {
                        self.fail().await;
                        //self.process_block(&db, &mut chain).await;
                    }
                }
            }

            self.wait(start).await;
        }
    }

    // Wait to the next cycle
    async fn wait(&mut self, start: tokio::time::Instant) {
        tracing::info!("Sleep to the nexc cycle");
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await
    }

    // If something fail
    async fn fail(&mut self) {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
    }

    async fn add_addresses(
        &mut self,
        db: &DatabaseConnection,
        chain_id: i32,
        address_list: &AddressList,
    ) {
        if address_list.is_empty() {
            return;
        }

        tracing::info!("Adding address to DB");
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH new_addresses as (SELECT unnest($2) as hash)
            
            INSERT INTO
                 address (chain, hash)
                 SELECT
                    $1, T.hash
                FROM
                    new_addresses T
                    LEFT JOIN address A
                        ON A.hash = T.hash AND A.chain = $1
                WHERE 
                    A.id IS NULL
            "#,
            vec![chain_id.into(), address_list.clone().into()],
        );

        db.execute(statement).await.unwrap();
    }

    async fn map_address(
        &mut self,
        db: &DatabaseConnection,
        chain_id: i32,
        addresses: &Vec<Vec<u8>>,
    ) -> BTreeMap<Vec<u8>, i64> {
        tracing::info!("Map addresses");

        let mut map = BTreeMap::new();
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH addresses as (SELECT unnest($2) as hash)
            
            SELECT
                A2.id, A2.hash
                
            FROM
                addresses A1
                LEFT JOIN address A2
                    ON A1.hash = A2.hash AND A2.chain = $1
            WHERE 
                    A2.id IS NOT NULL
            "#,
            vec![chain_id.into(), addresses.clone().into()],
        );

        if let Ok(result) = db.query_all(statement).await {
            for row in result {
                map.insert(
                    row.try_get("", "hash").unwrap(),
                    row.try_get("", "id").unwrap(),
                );
            }
        }
        map
    }

    async fn add_transactions(
        &mut self,
        db: &DatabaseConnection,
        chain_id: i32,
        transaction_list: TransactionList,
    ) {
        if transaction_list.is_empty() {
            return;
        }

        if let Ok(query) = crate::entity::transaction::Entity::find()
            .select_only()
            .column(crate::entity::transaction::Column::Hash)
            .filter(
                crate::entity::transaction::Column::Hash.is_in(
                    transaction_list
                        .iter()
                        .map(|t| t.0.clone())
                        .collect::<Vec<Vec<u8>>>(),
                ),
            )
            .all(db)
            .await
        {
            let transactions: BTreeSet<Vec<u8>> =
                BTreeSet::from_iter(query.iter().map(|t| t.hash.clone()));

            // Get all address bytes
            let address_list: Vec<Vec<u8>> = BTreeSet::from_iter(
                transaction_list
                    .iter()
                    .map(|t| t.2.iter().chain(t.3.iter()))
                    .flatten()
                    .map(|a| a.clone()),
            )
            .iter()
            .map(|a| a.clone())
            .collect();

            self.add_addresses(db, chain_id.clone(), &address_list)
                .await;

            // Transalte bytes to IDs
            let address_map = self.map_address(db, chain_id.clone(), &address_list).await;

            // Insert transactions
            if let Err(err) = crate::entity::transaction::Entity::insert_many(
                transaction_list
                    .iter()
                    .filter(|t| !transactions.contains(&t.0))
                    .map(|t| crate::entity::transaction::ActiveModel {
                        chain: Set(chain_id.clone()),
                        hash: Set(t.0.clone()),
                        amount: Set(if let Some(s) = t.1 {
                            Some(s as u32)
                        } else {
                            None
                        }),
                        from: Set(BTreeSet::from_iter(
                            t.2.iter()
                                .filter(|a| address_map.contains_key(a.deref()))
                                .map(|a| address_map.get(a).unwrap().clone()),
                        )
                        .into_iter()
                        .collect()),
                        to: Set(BTreeSet::from_iter(
                            t.3.iter()
                                .filter(|a| address_map.contains_key(a.deref()))
                                .map(|a| address_map.get(a).unwrap().clone()),
                        )
                        .into_iter()
                        .collect()),
                        ..Default::default()
                    })
                    .collect::<Vec<crate::entity::transaction::ActiveModel>>(),
            )
            .exec(db)
            .await
            {
                tracing::error!("{}", err.to_string());
            }
        }
    }

    async fn process_block(
        &mut self,
        db: &DatabaseConnection,
        chain: &mut crate::entity::chain::Model,
    ) {
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await;
        tracing::info!("Processing block for empty");
    }

    async fn process_address(&mut self, db: &DatabaseConnection, address: i64) {
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await;
        tracing::info!("Processing address fo empty");
    }
}
