use crate::entity::{address, service};
use crate::service::common::Service;
use crate::tag::Tag;
use async_trait::async_trait;
use sea_orm::{prelude::*, ConnectionTrait, DbBackend, Set, Statement, TryIntoModel};

#[derive(Clone, Debug)]
pub struct WingRiders {
    id: i32,
}

impl WingRiders {
    const TITLE: &str = "WingRiders";
    const SCRIPT_HASH: &str = "e6c90a5923713af5786963dee0fdffd830ca7e0c86a041d9e5833e91";
    const ADDRESS: &str = "7186ae9eebd8b97944a45201e4aec1330a72291af2d071644bba015959";

    fn tags() -> Vec<Tag> {
        vec![Tag::Dex]
    }

    pub async fn init(db: &DatabaseConnection, id: i32) -> Self {
        let service = match service::Entity::find_by_id(id).one(db).await.unwrap() {
            Some(result) => result,
            _ => {
                let new = service::ActiveModel {
                    id: Set(id),
                    title: Set(WingRiders::TITLE.into()),
                };

                new.save(db).await.unwrap().try_into_model().unwrap()
            }
        };

        Self { id: service.id }
    }
}

#[async_trait]
impl Service for WingRiders {
    async fn mark_addresses(&self, db: &DatabaseConnection) -> Result<(), String> {
        db.query_all(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            WITH
                wr_addresses AS (
                    SELECT 
                        id
                    FROM 
                        address 
                    WHERE 
                        -- Script hash
                        position(decode($2, 'hex') IN "hash") > 0
                        OR
                        -- Address
                        position(decode($3, 'hex') IN "hash") > 0
                        
                )
                UPDATE
                    address
                SET
                    services = array_unique(services || ARRAY[$1]),
                    tags = array_unique(tags || $3)
            "#,
            vec![
                self.id.into(),
                WingRiders::SCRIPT_HASH.into(),
                WingRiders::ADDRESS.into(),
                WingRiders::tags().into(),
            ],
        ))
        .await
        .unwrap();
        Ok(())
    }
}
