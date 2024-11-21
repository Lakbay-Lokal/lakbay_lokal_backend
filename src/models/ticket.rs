use sea_orm::entity::prelude::*;
use super::_entities::ticket::{ActiveModel, Entity};
pub type Ticket = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
