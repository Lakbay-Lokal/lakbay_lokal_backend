#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::auth::AuthorizationType;
use crate::models::_entities::user;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchUpdateUserLocation {
    pub user_id: i32,
    pub location_longitude: f64,
    pub location_latitude: f64,
}

#[debug_handler]
pub async fn update_user_location(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthorizationType>,
    Json(json): Json<PatchUpdateUserLocation>,
) -> Result<Response> {
    auth_type.validate_by_id(json.user_id)?;

    let Some(user) = user::Entity::find().one(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to find user: {:?}", e);
        e
    })?
    else {
        return Err(Error::NotFound);
    };

    let mut user = user.into_active_model();

    user.location_longitude = Set(json.location_longitude);
    user.location_latitude = Set(json.location_latitude);

    user.save(&ctx.db).await?;

    format::json(json!(
        {
            "message": "Location updated successfully"
        }
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users/tracking")
        .add("", patch(update_user_location))
}
