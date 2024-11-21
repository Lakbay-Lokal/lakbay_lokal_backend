use sea_orm::entity::prelude::*;
use super::_entities::bus_driver::{ActiveModel, Entity};
pub type BusDriver = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
