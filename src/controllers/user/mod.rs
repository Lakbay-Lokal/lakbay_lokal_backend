#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::middleware::auth::{AuthType};
use crate::models::_entities::user;
use axum::{debug_handler, Extension};
use loco_rs::prelude::*;
use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod tracking;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserInfoRequest {
    pub user_id: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserInfoResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub location_longitude: f64,
    pub location_latitude: f64,
    pub email: String,
    pub phone: String,
}

#[debug_handler]
pub async fn get_user_info(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthType>,
    Json(request): Json<GetUserInfoRequest>,
) -> Result<Response> {
    auth_type.validate_by_id(request.user_id)?;

    let Some(user) = user::Entity::find().one(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to find user: {:?}", e);
        e
    })?
    else {
        return Err(Error::NotFound);
    };

    format::json(&GetUserInfoResponse {
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        location_longitude: user.location_longitude,
        location_latitude: user.location_latitude,
        email: user.email,
        phone: user.phone,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutUserInfo {
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[debug_handler]
pub async fn put_user_info(
    State(ctx): State<AppContext>,
    Extension(auth_type): Extension<AuthType>,
    Json(json): Json<PutUserInfo>,
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

    if let Some(first_name) = json.first_name {
        user.first_name = Set(first_name);
    }

    if let Some(last_name) = json.last_name {
        user.last_name = Set(last_name);
    }

    if let Some(email) = json.email {
        user.email = Set(email);
    }

    if let Some(phone) = json.phone {
        user.phone = Set(phone);
    }

    user.save(&ctx.db).await?;

    format::json(json!({
        "message": "User info updated",
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users/")
        .add("/", get(get_user_info))
        .add("/", put(put_user_info))
}
