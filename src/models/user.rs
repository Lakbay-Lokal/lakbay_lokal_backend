use super::_entities::user::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type User = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
