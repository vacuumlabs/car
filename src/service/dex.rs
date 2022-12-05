use crate::entity::{address, service};
use crate::service::common::Service;
use crate::{common::Chain, tag::Tag};
use async_trait::async_trait;
use sea_orm::{prelude::*, ConnectionTrait, DbBackend, Set, Statement};

async fn init(db: &DatabaseConnection, id: i32, title: &str, _description: &str) -> service::Model {
    match service::Entity::find_by_id(id).one(db).await.unwrap() {
        Some(result) => result,
        None => {
            let new = service::ActiveModel {
                id: Set(id),
                title: Set(title.into()),
            };

            new.insert(db).await.unwrap()
        }
    }
}

async fn mark_script_hash(
    db: DatabaseConnection,
    chain: Chain,
    address: &str,
    service: i32,
    tags: Vec<Tag>,
) -> Result<(), String> {
    db.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        WITH
            addresses AS (
                SELECT 
                    id
                FROM 
                    address 
                WHERE 
                    -- Select chain
                    chain = $4
                    AND (
                        -- Address
                        position(decode($2, 'hex') IN "hash") = ANY(ARRAY[1, 2])
                    )
            )
        UPDATE
            address
        SET
            services = array_unique(services || ARRAY[$1]),
            tags = array_unique(tags || $3)
        WHERE address.id in (SELECT id from addresses) and chain = $4
        "#,
        vec![
            service.into(),
            address.into(),
            tags.iter()
                .map(|t| t.to_value())
                .collect::<Vec<i32>>()
                .into(),
            chain.to_value().into(),
        ],
    ))
    .await
    .unwrap();

    Ok(())
}

async fn mark_addresses(
    db: DatabaseConnection,
    chain: Chain,
    addresses: Vec<i64>,
    service: i32,
    tags: Vec<Tag>,
) -> Result<(), String> {
    db.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        UPDATE
            address
        SET
            services = array_unique(services || ARRAY[$1]),
            tags = array_unique(tags || $3)
        WHERE address.id = ANY($2) and chain = $4
        "#,
        vec![
            service.into(),
            addresses
                .iter()
                .map(|a| a.clone())
                .collect::<Vec<i64>>()
                .into(),
            tags.iter()
                .map(|t| t.to_value())
                .collect::<Vec<i32>>()
                .into(),
            chain.to_value().into(),
        ],
    ))
    .await
    .unwrap();

    Ok(())
}

macro_rules! dex {
    (
        name $name:ident;
        description $description:expr;
        tags $tags:expr;

        chain $chain:expr;
        pools $pools:expr;
        addresses $addresses:expr;
        other_script_hashes $other_script_hashes:expr;
    ) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            id: i32,
            chain: Chain,
        }
        impl $name {
            const TITLE: &str = stringify!($name);
            const DESCRIPTION: &str = $description;

            pub fn tags() -> Vec<Tag> {
                $tags
            }

            pub fn pools() -> Vec<&'static str> {
                $pools
            }

            pub fn addresses() -> Vec<&'static str> {
                $addresses
            }

            pub fn other_script_hashes() -> Vec<&'static str> {
                $other_script_hashes
            }

            pub async fn init(db: &DatabaseConnection, id: i32) -> Self {
                let service = init(db, id, $name::TITLE, $name::DESCRIPTION).await;

                Self {
                    id: service.id,
                    chain: $chain,
                }
            }
        }

        #[async_trait]
        impl Service for $name {
            async fn mark_addresses(&self, db: DatabaseConnection) -> Result<(), String> {
                let tags = $name::tags();

                let mut address_tags = tags.clone();
                address_tags.push(Tag::Address);
                for address in $name::addresses().iter() {
                    mark_script_hash(
                        db.clone(),
                        self.chain.clone(),
                        address,
                        self.id,
                        address_tags.clone(),
                    )
                    .await
                    .unwrap()
                }

                let mut pool_tags = tags.clone();
                pool_tags.push(Tag::Pool);
                for address in $name::pools().iter() {
                    mark_script_hash(
                        db.clone(),
                        self.chain.clone(),
                        address,
                        self.id,
                        pool_tags.clone(),
                    )
                    .await
                    .unwrap();
                }

                for address in $name::other_script_hashes().iter() {
                    mark_script_hash(
                        db.clone(),
                        self.chain.clone(),
                        address,
                        self.id,
                        tags.clone(),
                    )
                    .await
                    .unwrap();
                }

                Ok(())
            }
        }
    };
}

dex!(
    name WingRiders;
    description "";
    tags vec![Tag::Dex, Tag::Finance];

    chain Chain::Cardano;
    pools vec!["e6c90a5923713af5786963dee0fdffd830ca7e0c86a041d9e5833e91"];
    addresses vec!["7186ae9eebd8b97944a45201e4aec1330a72291af2d071644bba015959"];
    other_script_hashes Vec::new();
);

dex!(
    name SundaeSwap;
    description "";
    tags vec![Tag::Dex, Tag::Finance];

    chain Chain::Cardano;
    pools vec!["4020e7fc2de75a0729c3cc3af715b34d98381e0cdbcfa99c950bc3ac"];
    addresses vec!["714020e7fc2de75a0729c3cc3af715b34d98381e0cdbcfa99c950bc3ac"];
    other_script_hashes Vec::new();
);

dex!(
    name MinSwap;
    description "";
    tags vec![Tag::Dex, Tag::Finance];

    chain Chain::Cardano;
    pools vec![
        "57c8e718c201fba10a9da1748d675b54281d3b1b983c5d1687fc7317",
        "e1317b152faac13426e6a83e06ff88a4d62cce3c1634ab0a5ec13309"
    ];
    addresses vec![
        "710ca50950adbb06c5c5f6833924a66ac873e43202588b6d338602d78d",
        "71a65ca58a4e9c755fa830173d2a5caed458ac0c73f97db7faae2e7e3b"
    ];
    other_script_hashes Vec::new();
);
