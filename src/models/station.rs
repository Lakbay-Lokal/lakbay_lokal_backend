use sea_orm::entity::prelude::*;
use super::_entities::station::{ActiveModel, Entity};
pub type Station = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
