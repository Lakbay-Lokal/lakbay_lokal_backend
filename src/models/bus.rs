use sea_orm::entity::prelude::*;
use super::_entities::bus::{ActiveModel, Entity};
pub type Bus = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
