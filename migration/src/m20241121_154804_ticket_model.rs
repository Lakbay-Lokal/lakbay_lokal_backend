use crate::m20241121_154531_driver_model::BusDriver;
use crate::m20241121_155207_bus_model::Bus;
use crate::m20241121_160948_station_model::Station;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Ticket {
    Table,
    Id,
    NumberOfCommuters,
    NumberOfDiscountedCommuters,
    TotalAmount,
    PaymentMethod,

    // Foreign keys
    OriginStationId,
    DestinationStationId,
    BusId,
    DriverId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Ticket::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Ticket::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(Ticket::NumberOfCommuters)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Ticket::NumberOfDiscountedCommuters)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Ticket::TotalAmount).double().not_null())
            .col(
                ColumnDef::new(Ticket::PaymentMethod)
                    .string_len(255)
                    .not_null(),
            )
            .col(ColumnDef::new(Ticket::BusId).integer().not_null())
            .col(ColumnDef::new(Ticket::DriverId).integer().not_null())
            .col(ColumnDef::new(Ticket::OriginStationId).integer().not_null())
            .col(
                ColumnDef::new(Ticket::DestinationStationId)
                    .integer()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_bus_id")
                    .from(Ticket::Table, Ticket::BusId)
                    .to(Bus::Table, Bus::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_driver_id")
                    .from(Ticket::Table, Ticket::DriverId)
                    .to(BusDriver::Table, BusDriver::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_origin_station_id")
                    .from(Ticket::Table, Ticket::OriginStationId)
                    .to(Station::Table, Station::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_destination_station_id")
                    .from(Ticket::Table, Ticket::DestinationStationId)
                    .to(Station::Table, Station::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            // .index(Index::create().name("idx_ticket_bus").col(Ticket::BusId))
            // .index(Index::create().name("idx_ticket_driver").col(Ticket::DriverId))
            // .index(Index::create().name("idx_ticket_origin_station").col(Ticket::OriginStationId))
            // .index(Index::create().name("idx_ticket_destination_station").col(Ticket::DestinationStationId))
            .to_owned();

        println!("{}", table.build(PostgresQueryBuilder));
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ticket::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}
