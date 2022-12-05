use sea_orm::{
    ActiveEnum, ConnectionTrait, DatabaseConnection, DbBackend, DeriveActiveEnum, EnumIter,
    Statement,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Chain {
    #[sea_orm(num_value = 1)]
    Cardano,
    #[sea_orm(num_value = 2)]
    Ethereum,
    #[sea_orm(num_value = 3)]
    Polygon,
}

pub enum DirectionOfInteraction {
    To,
    From,
}

pub async fn address_interacting(
    db: DatabaseConnection,
    chains: Vec<Chain>,
    addresses_from: Option<Vec<i64>>,
    addresses_to: Option<Vec<i64>>,
    services: Option<Vec<i32>>,
    tags: Option<Vec<i32>>,
    direction: DirectionOfInteraction,
) -> Result<BTreeSet<i64>, String> {
    let sql = {
        let sql_services = match &services {
            Some(s) if !s.is_empty() => "AND A.services && $2",
            _ => "",
        };
        let sql_tags = match &tags {
            Some(t) if !t.is_empty() => "AND A.tags && $3",
            _ => "",
        };
        let sql_address_from = match &addresses_from {
            Some(af) if !af.is_empty() => r#"AND T."from" && $4"#,
            _ => "",
        };
        let sql_address_to = match &addresses_to {
            Some(at) if !at.is_empty() => r#"AND T."to" && $5"#,
            _ => "",
        };
        let sql_column = match &direction {
            DirectionOfInteraction::From => "from",
            DirectionOfInteraction::To => "to",
        };

        let sql_where = match &direction {
            DirectionOfInteraction::From => "to",
            DirectionOfInteraction::To => "from",
        };
        format!(
            r#"
                SELECT 
                    T."{sql_column}" as addresses
                FROM
                    transaction T 
                WHERE 
                    T."{sql_where}" && (
                        SELECT 
                            array_agg(A.id)
                        FROM 
                            address A 
                        WHERE 
                            A.chain = ANY($1)
                            {sql_services}
                            {sql_tags}
                    )
                    AND
                    T.chain = ANY($1)
                    {sql_address_from}
                    {sql_address_to}
                    "#
        )
    };

    let statement = Statement::from_sql_and_values(
        DbBackend::Postgres,
        &sql,
        vec![
            chains
                .iter()
                .map(|c| c.to_value())
                .collect::<Vec<i32>>()
                .into(),
            services.into(),
            tags.iter()
                .flatten()
                .map(|t| t.clone())
                .collect::<Vec<i32>>()
                .into(),
            addresses_from
                .iter()
                .flatten()
                .map(|a| a.clone())
                .collect::<Vec<i64>>()
                .into(),
            addresses_to
                .iter()
                .flatten()
                .map(|a| a.clone())
                .collect::<Vec<i64>>()
                .into(),
        ],
    );

    match db.query_all(statement).await {
        Ok(query) => {
            let mut result = BTreeSet::new();

            for row in query.iter() {
                for addresses in row.try_get::<Vec<i64>>("", "addresses") {
                    for address in addresses.iter() {
                        result.insert(address.clone());
                    }
                }
            }
            return Ok(result);
        }
        _ => Err(String::from("SQL query failed")),
    }
}
