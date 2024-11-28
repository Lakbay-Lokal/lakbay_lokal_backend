#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::middleware::auth::AuthorizationType;
use crate::models::_entities::bus;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde_json::json;

#[debug_handler]
pub async fn get_all_bus_location(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
) -> Result<Response> {
    auth_type.validate()?;

    let bus = bus::Entity::find().all(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to find bus: {:?}", e);
        e
    })?;

    let mut locations = vec![];

    for b in bus {
        locations.push(json!({
            "bus_id": b.id,
            "location_longitude": b.location_longitude,
            "location_latitude": b.location_latitude,
        }));
    }

    format::json(json!(
        {
            "locations": locations
        }
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("bus/tracking/all")
        .add("", get(get_all_bus_location))
}
