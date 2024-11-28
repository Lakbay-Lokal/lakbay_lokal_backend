#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::auth::{AuthorizationType, BusAuthorizationType};
use crate::models::_entities::bus;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchBusLocationRequest {
    pub bus_id: i32,
    pub location_longitude: f64,
    pub location_latitude: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBusLocationRequest {
    pub bus_id: i32,
}

#[debug_handler]
pub async fn update_bus_location(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<BusAuthorizationType>,
    Json(json): Json<PatchBusLocationRequest>,
) -> Result<Response> {
    auth_type.validate_by_id(json.bus_id)?;

    let Some(bus) = bus::Entity::find().one(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to find bus: {:?}", e);
        e
    })?
    else {
        return Err(Error::NotFound);
    };

    let mut bus = bus.into_active_model();

    bus.location_longitude = Set(json.location_longitude);
    bus.location_latitude = Set(json.location_latitude);

    bus.save(&ctx.db).await?;

    format::json(json!(
        {
            "message": "Location updated successfully"
        }
    ))
}

#[debug_handler]
pub async fn get_bus_location_by_id(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
    Json(request): Json<GetBusLocationRequest>,
) -> Result<Response> {
    auth_type.validate_by_id(request.bus_id)?;

    let Some(bus) = bus::Entity::find().one(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to find bus: {:?}", e);
        e
    })?
    else {
        return Err(Error::NotFound);
    };

    format::json(json!(
        {
            "location_longitude": bus.location_longitude,
            "location_latitude": bus.location_latitude,
        }
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("bus/tracking")
        .add("", patch(update_bus_location))
        .add("", get(get_bus_location_by_id))
}
