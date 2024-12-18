//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "station")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Double")]
    pub location_longitude: f64,
    #[sea_orm(column_type = "Double")]
    pub location_latitude: f64,
    pub number_of_users_near: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bus::Entity")]
    Bus,
}

impl Related<super::bus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bus.def()
    }
}
