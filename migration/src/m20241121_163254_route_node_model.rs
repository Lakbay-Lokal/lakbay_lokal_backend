use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241121_160948_station_model::Station;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum RouteNode {
    Table,
    Id,
    IsStartOfRoute,
    RouteId,

    // Foreign keys
    OriginStation,
    DestinationStation,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(RouteNode::Table)
            .col(
                ColumnDef::new(RouteNode::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(
                ColumnDef::new(RouteNode::IsStartOfRoute)
                    .boolean()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RouteNode::RouteId)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RouteNode::OriginStation)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(RouteNode::DestinationStation)
                    .integer()
                    .not_null(),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_route_id")
                    .from(RouteNode::Table, RouteNode::RouteId)
                    .to(Station::Table, Station::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_origin_station")
                    .from(RouteNode::Table, RouteNode::OriginStation)
                    .to(Station::Table, Station::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_destination_station")
                    .from(RouteNode::Table, RouteNode::DestinationStation)
                    .to(Station::Table, Station::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            // .index(Index::create().name("idx_origin_station").col(RouteNode::OriginStation))
            // .index(Index::create().name("idx_destination_station").col(RouteNode::DestinationStation))
            // .index(Index::create().name("idx_next_route_node").col(RouteNode::NextRouteNode))
            .to_owned();

        println!("{}", table.build(PostgresQueryBuilder));
        manager.create_table(
            table
        )
                .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop()
                .table(RouteNode::Table)
                .if_exists()
                .to_owned()
        )
                .await?;
        Ok(())
    }
}

