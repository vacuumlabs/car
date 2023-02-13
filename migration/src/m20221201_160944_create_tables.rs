use sea_orm_migration::prelude::*;
use std::sync::Arc;

#[derive(DeriveMigrationName)]
pub struct Migration;

// Use GIN index type
#[derive(Clone, Debug, PartialEq, sea_query::Iden, sea_schema_derive::Name)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum CustomIndexType {
    #[iden = "GIN"]
    Gin,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Chain - enumerator
        manager
            .create_table(
                Table::create()
                    .table(Chain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chain::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chain::Title).string().not_null())
                    .col(ColumnDef::new(Chain::Params).json().not_null())
                    .to_owned(),
            )
            .await?;

        // Service
        manager
            .create_table(
                Table::create()
                    .table(Service::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Service::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Service::Title).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Tag
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tag::Title).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Address
        manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::Chain).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chain-id")
                            .from(Address::Table, Address::Chain)
                            .to(Chain::Table, Chain::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Address::Hash).binary().not_null())
                    .col(ColumnDef::new(Address::Title).string().null())
                    .col(
                        ColumnDef::new(Address::Services)
                            .array(ColumnType::Integer(None))
                            .not_null()
                            .default(SimpleExpr::Custom(String::from("Array[]::integer[]"))),
                    )
                    .col(
                        ColumnDef::new(Address::Tags)
                            .array(ColumnType::Integer(None))
                            .not_null()
                            .default(SimpleExpr::Custom(String::from("Array[]::integer[]"))),
                    )
                    .to_owned(),
            )
            .await?;
        {
            // Indexes
            manager
                .create_index(
                    Index::create()
                        .name("address-idx-chain")
                        .table(Address::Table)
                        .col(Address::Chain)
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("address-idx-hash")
                        .table(Address::Table)
                        .col(Address::Hash)
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("address-idx-services")
                        .table(Address::Table)
                        .col(Address::Services)
                        .index_type(IndexType::Custom(Arc::new(CustomIndexType::Gin)))
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("address-idx-tags")
                        .table(Address::Table)
                        .col(Address::Tags)
                        .index_type(IndexType::Custom(Arc::new(CustomIndexType::Gin)))
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("address-unique")
                        .table(Address::Table)
                        .col(Address::Hash)
                        .col(Address::Chain)
                        .unique()
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::Chain).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chain-id")
                            .from(Transaction::Table, Transaction::Chain)
                            .to(Chain::Table, Chain::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Transaction::Hash).binary().not_null())
                    .col(ColumnDef::new(Transaction::Amount).big_unsigned().null())
                    .col(
                        ColumnDef::new(Transaction::From)
                            .array(ColumnType::BigInteger(None))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transaction::To)
                            .array(ColumnType::BigInteger(None))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        {
            // Indexes
            manager
                .create_index(
                    Index::create()
                        .name("transaction-idx-chain")
                        .table(Transaction::Table)
                        .col(Transaction::Chain)
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("transaction-idx-hash")
                        .table(Transaction::Table)
                        .col(Transaction::Hash)
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("transaction-idx-from")
                        .table(Transaction::Table)
                        .col(Transaction::From)
                        .index_type(IndexType::Custom(Arc::new(CustomIndexType::Gin)))
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("transaction-idx-to")
                        .table(Transaction::Table)
                        .col(Transaction::To)
                        .index_type(IndexType::Custom(Arc::new(CustomIndexType::Gin)))
                        .to_owned(),
                )
                .await?;
            manager
                .create_index(
                    Index::create()
                        .name("transaction-unique")
                        .table(Transaction::Table)
                        .col(Transaction::Hash)
                        .col(Transaction::Chain)
                        .unique()
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Chain::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Address::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Transaction::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Service::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Tag::Table)
                    .if_exists()
                    .cascade()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Chain {
    Table,
    Id,
    Title,
    Params,
}

#[derive(Iden)]
enum Address {
    Table,
    Id,
    Chain,
    Hash,
    Title,
    Services,
    Tags,
}

#[derive(Iden)]
enum Transaction {
    Table,
    Id,
    Chain,
    Hash,
    Amount,
    From,
    To,
}

#[derive(Iden)]
enum Service {
    Table,
    Id,
    Title,
}

#[derive(Iden)]
enum Tag {
    Table,
    Id,
    Title,
}
