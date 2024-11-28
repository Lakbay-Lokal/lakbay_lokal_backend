#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::auth::AuthorizationType;
use crate::models::_entities::bus;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod tracking;
pub mod tracking_all;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBusRequest {
    pub route_id: i32,
    pub next_station_id: i32,
    pub assigned_driver: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchBusRequest {
    pub bus_id: i32,
    pub route_id: Option<i32>,
    pub next_station_id: Option<i32>,
    pub number_of_people_inside: Option<i32>,
    pub assigned_driver: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBusRequest {
    pub bus_id: i32,
}

#[debug_handler]
pub async fn create_bus_endpoint(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
    Json(json): Json<CreateBusRequest>,
) -> Result<Response> {
    auth_type.admin_only()?;

    let bus = bus::ActiveModel {
        route_id: Set(json.route_id),
        next_station_id: Set(json.next_station_id),
        assigned_driver: Set(json.assigned_driver),
        ..Default::default()
    };

    bus.save(&ctx.db).await?;

    format::json(json!({
        "message": "Bus created successfully"
    }))
}

#[debug_handler]
pub async fn update_bus_endpoint(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
    Json(json): Json<PatchBusRequest>,
) -> Result<Response> {
    auth_type.admin_only()?;

    let bus = bus::Entity::find()
        .filter(bus::Column::Id.eq(json.bus_id))
        .one(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find bus: {:?}", e);
            e
        })?
        .ok_or(Error::NotFound)?;

    let mut bus = bus.into_active_model();

    if let Some(route_id) = json.route_id {
        bus.route_id = Set(route_id);
    }

    if let Some(next_station_id) = json.next_station_id {
        bus.next_station_id = Set(next_station_id);
    }

    if let Some(number_of_people_inside) = json.number_of_people_inside {
        bus.number_of_people_inside = Set(number_of_people_inside);
    }

    if let Some(assigned_driver) = json.assigned_driver {
        bus.assigned_driver = Set(assigned_driver);
    }

    bus.save(&ctx.db).await?;

    format::json(json!({
        "message": "Bus updated successfully"
    }))
}

#[debug_handler]
pub async fn get_bus_endpoint(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
    Json(json): Json<GetBusRequest>,
) -> Result<Response> {
    auth_type.validate()?;

    let bus = bus::Entity::find()
        .filter(bus::Column::Id.eq(json.bus_id))
        .one(&ctx.db)
        .await?;

    format::json(bus)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("bus")
        .add("/", post(create_bus_endpoint))
        .add("/", get(get_bus_endpoint))
        .add("/", patch(update_bus_endpoint))
}
