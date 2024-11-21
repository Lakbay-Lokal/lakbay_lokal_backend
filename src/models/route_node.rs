use sea_orm::entity::prelude::*;
use super::_entities::route_node::{ActiveModel, Entity};
pub type RouteNode = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
