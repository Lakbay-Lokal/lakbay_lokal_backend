use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Station {
    Table,
    Id,
    LocationLongitude,
    LocationLatitude,
    NumberOfUsersNear,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Station::Table)
                    .col(
                        ColumnDef::new(Station::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Station::LocationLongitude)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Station::LocationLatitude)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Station::NumberOfUsersNear)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Station::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}
