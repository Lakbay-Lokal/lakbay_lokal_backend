use super::_entities::station::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Station = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
