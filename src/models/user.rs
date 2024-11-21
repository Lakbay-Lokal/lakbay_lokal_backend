use sea_orm::entity::prelude::*;
use super::_entities::user::{ActiveModel, Entity};
pub type User = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}