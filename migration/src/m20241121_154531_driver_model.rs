use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum BusDriver {
    Table,
    Id,
    Name,
    HomeAddress,
    Phone,
    Email,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(Table::create()
                .table(BusDriver::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(BusDriver::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment()
                )
                .col(
                    ColumnDef::new(BusDriver::Name)
                        .string_len(255)
                        .not_null()
                )
                .col(
                    ColumnDef::new(BusDriver::HomeAddress)
                        .string_len(255)
                        .not_null()
                )
                .col(
                    ColumnDef::new(BusDriver::Phone)
                        .string_len(255)
                        .not_null()
                )
                .col(
                    ColumnDef::new(BusDriver::Email)
                        .string_len(255)
                        .not_null()
                )
                .to_owned()
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(BusDriver::Table)
                .if_exists()
                .to_owned()
            )
            .await?;
        Ok(())
    }
}

