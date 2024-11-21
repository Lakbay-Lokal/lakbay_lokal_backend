//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "route_node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub is_start_of_route: bool,
    pub route_id: i32,
    pub origin_station: i32,
    pub destination_station: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::station::Entity",
        from = "Column::DestinationStation",
        to = "super::station::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Station3,
    #[sea_orm(
        belongs_to = "super::station::Entity",
        from = "Column::OriginStation",
        to = "super::station::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Station2,
    #[sea_orm(
        belongs_to = "super::station::Entity",
        from = "Column::RouteId",
        to = "super::station::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Station1,
}
