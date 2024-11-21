#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20241121_153310_user_model;
mod m20241121_154531_driver_model;
mod m20241121_154804_ticket_model;
mod m20241121_155207_bus_model;
mod m20241121_160948_station_model;
mod m20241121_163254_route_node_model;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // inject-below (do not remove this comment)
            Box::new(m20241121_153310_user_model::Migration),
            Box::new(m20241121_160948_station_model::Migration),
            Box::new(m20241121_154531_driver_model::Migration),
            Box::new(m20241121_155207_bus_model::Migration),
            Box::new(m20241121_154804_ticket_model::Migration),
            Box::new(m20241121_163254_route_node_model::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}