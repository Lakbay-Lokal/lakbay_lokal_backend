use super::_entities::bus::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Bus = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
