use super::_entities::route_node::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type RouteNode = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
