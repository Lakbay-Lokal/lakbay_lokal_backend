use super::_entities::ticket::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
pub type Ticket = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
