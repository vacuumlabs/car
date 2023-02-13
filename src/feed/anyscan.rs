use crate::feed::Feed;
use async_trait::async_trait;
use rweb::http::request;
use sea_orm::{entity::*, query::*, ActiveModelTrait, DatabaseConnection, Set, Unset};

fn to_transaction(transactions: &Vec<serde_json::Value>) -> super::TransactionList {
    transactions
        .iter()
        .map(|t| {
            (
                hex::decode(
                    &t["hash"]
                        .as_str()
                        .unwrap_or("0000")
                        .chars()
                        .skip(2)
                        .collect::<String>(),
                )
                .unwrap(),
                if let Some(value) = t["value"].as_str() {
                    Some(
                        u128::from_str_radix(&value.chars().skip(2).collect::<String>(), 16)
                            .unwrap_or(0),
                    )
                } else {
                    None
                },
                vec![hex::decode(
                    t["from"]
                        .as_str()
                        .unwrap_or("0x0000")
                        .chars()
                        .skip(2)
                        .collect::<String>(),
                )
                .unwrap()],
                vec![hex::decode(
                    t["to"]
                        .as_str()
                        .unwrap_or("0x0000")
                        .chars()
                        .skip(2)
                        .collect::<String>(),
                )
                .unwrap()],
            )
        })
        .collect()
}

async fn action(
    any: &mut shared::AnyScan,
    db: &DatabaseConnection,
    address: &crate::entity::address::Model,
    action: &str,
) {
    let mut block = 0;
    let mut start = tokio::time::Instant::now();

    loop {
        let url = format!(
            "{}?module=account&action={}&address=0x{}&startblock={}&apikey={}",
            any.base_url,
            action,
            hex::encode(&address.hash),
            block,
            any.token
        );
        tracing::info!("AnyScan Address: {}", url);
        start = tokio::time::Instant::now();

        if let Ok(request) = reqwest::get(url).await {
            if let Ok(body) = request.json::<serde_json::Value>().await {
                if let Some(transactions) = body["result"].as_array() {
                    any.add_transactions(
                        db,
                        address.chain.clone(),
                        // Iterate over result values
                        to_transaction(transactions),
                    )
                    .await;

                    // Id the result is full,
                    if transactions.len() == 10000 {
                        block = transactions
                            .iter()
                            .map(|t| {
                                t["blockNumber"]
                                    .as_str()
                                    .unwrap_or(&block.to_string())
                                    .parse::<u64>()
                                    .unwrap_or(block)
                            })
                            .fold(block + 1, |max, i| std::cmp::max(max, i));
                        continue;
                    }
                }
            }
        }
        break;
    }
}

#[async_trait]
impl super::Feed for shared::AnyScan {
    async fn wait(&mut self, start: tokio::time::Instant) {
        tracing::info!("AnyScan waiting");
        let delay = tokio::time::Instant::now().duration_since(start);
        if let Some(duration) = tokio::time::Duration::from_millis(self.delay).checked_sub(delay) {
            tracing::info!("AnyScan sleeping for: {}", duration.as_millis());
            tokio::time::sleep(duration).await;
        } else {
            tracing::info!("Evil loop!!! {}", delay.as_millis());
        }
    }

    async fn process_block(
        &mut self,
        db: &DatabaseConnection,
        chain: &mut crate::entity::chain::Model,
    ) {
        // Query string for *Scan apis
        let url = format!(
            "{}?module=proxy&action=eth_getBlockByNumber&tag={:#0x}&boolean=true&apikey={}",
            self.base_url, self.last, self.token
        );
        tracing::info!("AnyScan block: {}", url);

        if let Ok(request) = reqwest::get(url).await {
            if let Ok(body) = request.json::<serde_json::Value>().await {
                if let Some(result) = body["result"].as_object() {
                    if let Some(transactions) = result["transactions"].as_array() {
                        self.add_transactions(
                            db,
                            chain.id.clone(),
                            // Iterate over result values
                            to_transaction(transactions),
                        )
                        .await;
                    }

                    if let Some(last) = result["number"].as_str() {
                        self.last = u64::from_str_radix(&last[2..], 16).unwrap() + 1;

                        let mut params: shared::ChainParam =
                            serde_json::from_value(chain.params.clone()).unwrap();
                        match &mut params {
                            shared::ChainParam::ArbiScan(arbi) => arbi.last = self.last.clone(),
                            shared::ChainParam::EtherScan(ether) => ether.last = self.last.clone(),
                            shared::ChainParam::PolyScan(ether) => ether.last = self.last.clone(),
                            _ => {}
                        }
                        let chain_update = crate::entity::chain::ActiveModel {
                            id: Set(chain.id.clone()),
                            params: Set(serde_json::to_value(params).unwrap()),
                            title: Unset(None),
                        };

                        if let Ok(c) = chain_update.update(db).await {
                            tracing::info!("Chain updated");
                            chain.params = serde_json::to_value(self.clone()).unwrap().clone();
                        } else {
                            tracing::info!("Chain not updated");
                        }
                    }
                }
            } else {
                tracing::error!("Problem with deserialize");
            }
        } else {
            tracing::error!("Problem with request");
        }
    }

    async fn process_address(&mut self, db: &DatabaseConnection, address: i64) {
        if let Ok(Some(address)) = crate::entity::address::Entity::find_by_id(address)
            .column(crate::entity::address::Column::Hash)
            .column(crate::entity::address::Column::Chain)
            .one(db)
            .await
        {
            // Query string for *Scan apis
            action(self, db, &address, "txlist").await;
            action(self, db, &address, "txlistinternal").await;
        } else {
            tracing::error!("Address not found!!!!");
        }
    }
}
