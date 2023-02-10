use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Set, Statement};
use std::collections::BTreeMap;
use tokio::sync::mpsc::Receiver;

mod anyscan;

#[derive(Clone, Debug)]
pub enum FeedCommand {
    Address(i64),
    Stop,
}

type AddressList = Vec<Vec<u8>>;
type TransactionList = Vec<(Vec<u8>, Option<i64>, Vec<i64>, Vec<i64>)>;

#[async_trait]
pub trait Feed {
    async fn run(
        &self,
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
                if let Some(chain) = result {
                    if let Ok(msg) = receiver.try_recv() {
                        match msg {
                            FeedCommand::Address(a) => self.process_address(&db, a).await,
                            FeedCommand::Stop => break,
                        }
                    } else {
                        self.process_block(&db, &chain).await;
                    }
                }
            }

            self.wait(start).await;
        }
    }

    // Wait to the next cycle
    async fn wait(&self, start: tokio::time::Instant) {
        tracing::info!("Sleep to the nexc cycle");
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await
    }

    // If something fail
    async fn fail(&self) {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
    }

    async fn add_addresses(
        &self,
        db: &DatabaseConnection,
        chain_id: i32,
        address_list: AddressList,
    ) {
        tracing::info!("Adding address to DB");
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH new_addresses as (SELECT chain, hash FROM unnest($1, $2) as data(chain, hash))
            
            INSERT INTO
                 address (chain, hash)
                 SELECT
                    T.chain, T.hash
                FROM
                    new_addresses T
                    LEFT JOIN address A
                        ON A.hash = T.hash AND A.chain = T.chain
                WHERE 
                    A.id IS NULL
            "#,
            vec![
                vec![chain_id.clone(); address_list.len()].into(),
                address_list.into(),
            ],
        );

        db.execute(statement).await.unwrap();
    }

    async fn map_address(
        &self,
        db: &DatabaseConnection,
        chain_id: i32,
        addresses: Vec<Vec<u8>>,
    ) -> BTreeMap<Vec<u8>, i64> {
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH addresses as (SELECT chain, hash FROM unnest($1, $2) as data(chain, hash))
            
            SELECT
                A1.id, A1.hash
                
                FROM
                address A1
                    LEFT JOIN addresses A2
                        ON A1.hash = A2.hash AND A1.chain = A2.chain
                WHERE 
                    A1.id IS NOT NULL
            "#,
            vec![
                vec![chain_id.clone(); addresses.len()].into(),
                addresses.into(),
            ],
        );

        if let Ok(result) = db.query_all(statement).await {
            //for let Some(row) in result {
            //}
        }

        BTreeMap::new()
    }

    async fn add_transactions(
        &self,
        db: &DatabaseConnection,
        chain_id: i32,
        transaction_list: TransactionList,
    ) {
        tracing::info!("Adding address to DB");
        // select values to SKIP

        let statement = crate::entity::transaction::Entity::insert_many(
            transaction_list
                .iter()
                .filter(|t| true)
                .map(|t| crate::entity::transaction::ActiveModel {
                    chain: Set(chain_id.clone()),
                    hash: Set(t.0.clone()),
                    amount: Set(t.1.clone()),
                    from: Set(t.2.clone()),
                    to: Set(t.3.clone()),
                    ..Default::default()
                })
                .collect::<Vec<crate::entity::transaction::ActiveModel>>(),
        );
        statement.exec(db).await;
    }

    async fn process_block(&self, db: &DatabaseConnection, chain: &crate::entity::chain::Model) {
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await;
        tracing::info!("Processing block");
    }

    async fn process_address(&self, db: &DatabaseConnection, address: i64) {
        tokio::time::sleep(tokio::time::Duration::from_millis(5_000)).await;
        tracing::info!("Processing address");
    }
}
