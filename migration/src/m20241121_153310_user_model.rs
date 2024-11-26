use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    FirstName,
    LastName,
    LocationLongitude,
    LocationLatitude,
    Email,
    Phone,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string_len(255).not_null())
                    .col(ColumnDef::new(User::FirstName).string_len(255).not_null())
                    .col(ColumnDef::new(User::LastName).string_len(255).not_null())
                    .col(ColumnDef::new(User::LocationLongitude).double().not_null())
                    .col(ColumnDef::new(User::LocationLatitude).double().not_null())
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(User::Phone)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}
