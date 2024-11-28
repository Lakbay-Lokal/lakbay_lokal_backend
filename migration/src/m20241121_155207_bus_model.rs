use crate::m20241121_154531_driver_model::BusDriver;
use crate::m20241121_160948_station_model::Station;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

// Bus
// LocationLongitude
// LocationLatitude
// Route ID
// Next Station ID
// Number of People inside
// Assigned Driver
// History: Vec<Ticket>

#[derive(DeriveIden)]
pub enum Bus {
    Table,
    Id,
    LocationLongitude,
    LocationLatitude,
    NumberOfPeopleInside,
    IsRunning,
    RouteId,
    // Foreign keys
    NextStationId,
    AssignedDriver,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Bus::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Bus::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Bus::LocationLongitude).double().not_null())
            .col(ColumnDef::new(Bus::LocationLatitude).double().not_null())
            .col(
                ColumnDef::new(Bus::NumberOfPeopleInside)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(Bus::IsRunning).boolean().not_null())
            .col(ColumnDef::new(Bus::RouteId).integer().not_null())
            .col(ColumnDef::new(Bus::NextStationId).integer().not_null())
            .col(ColumnDef::new(Bus::AssignedDriver).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_next_station_id")
                    .from(Bus::Table, Bus::NextStationId)
                    .to(Station::Table, Station::Id),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_assigned_driver")
                    .from(Bus::Table, Bus::AssignedDriver)
                    .to(BusDriver::Table, BusDriver::Id),
            )
            // .index(Index::create().name("idx_bus_route").col(Bus::RouteId))
            // .index(Index::create().name("idx_bus_next_station").col(Bus::NextStationId))
            // .index(Index::create().name("idx_bus_assigned_driver").col(Bus::AssignedDriver))
            .to_owned();

        println!("{}", table.build(PostgresQueryBuilder));
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bus::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}
