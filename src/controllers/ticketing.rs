#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::{debug_handler, Extension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::middleware::auth::{BusAuthType};
use crate::models;
use crate::models::_entities::ticket;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBusTicketRequest {
    number_of_commuters: i32,
    number_of_discounted_commuters: i32,
    total_amount: f64,
    payment_method: String,
    bus_id: i32,
    origin_station_id: i32,
    destination_station_id: i32,
}

#[debug_handler]
pub async fn create_bus_ticket(
    State(ctx): State<AppContext>,
    Extension(bus_auth): Extension<BusAuthType>,
    Json(json): Json<CreateBusTicketRequest>,
) -> Result<Response> {
    bus_auth.validate_by_id(json.bus_id)?;

    // active model
    let ticket = ticket::ActiveModel {
        number_of_commuters: Set(json.number_of_commuters),
        number_of_discounted_commuters: Set(json.number_of_discounted_commuters),
        total_amount: Set(json.total_amount),
        payment_method: Set(json.payment_method),
        bus_id: Set(json.bus_id),
        origin_station_id: Set(json.origin_station_id),
        destination_station_id: Set(json.destination_station_id),
        ..Default::default()
    };

    ticket.save(&ctx.db).await?;


    format::json(
        json!({
            "message": "Ticket created successfully"
        })
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("ticketing/")
        .add("/", post(create_bus_ticket))
}
