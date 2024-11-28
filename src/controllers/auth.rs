#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::user;
use crate::models::auth::UserClaims;
use axum::debug_handler;
use loco_rs::prelude::*;
use sea_orm::Condition;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::ops::Add;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::empty()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    phone_number: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    user_id: i32,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    phone_number: String,
}

#[debug_handler]
pub async fn register(
    State(ctx): State<AppContext>,
    Json(json): Json<RegisterRequest>,
) -> Result<Response> {
    // Should probably use the Appropriate　HTTP status code

    // Check if user already exists, if username exists, email exists, phone number exists, return error
    if let Some(_user) = user::Entity::find()
        // use or
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(json.username.clone()))
                .add(user::Column::Email.eq(json.email.clone()))
                .add(user::Column::Phone.eq(json.phone_number.clone())),
        )
        .one(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find user: {:?}", e);
            e
        })?
    {
        return format::json(json!({
            "error": "User already exists",
        }));
    }

    let user = user::ActiveModel {
        first_name: Set(json.first_name.clone()),
        last_name: Set(json.last_name.clone()),
        username: Set(json.username.clone()),
        email: Set(json.email.clone()),
        phone: Set(json.phone_number.clone()),
        password: Set(json.password.clone()),
        ..Default::default()
    };

    user.insert(&ctx.db).await.map_err(|e| {
        tracing::error!("Failed to insert user: {:?}", e);
        e
    })?;

    let user: user::Model = user::Entity::find()
        .filter(user::Column::Username.eq(json.username))
        .one(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find user, may not have been created: {:?}", e);
            e
        })?
        .ok_or_else(|| {
            tracing::error!("Failed to find user, may not have been created");
            Error::NotFound
        })?;

    format::json(&RegisterResponse {
        user_id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        username: user.username,
        email: user.email,
        phone_number: user.phone,
    })
}

#[derive(Debug)]
pub enum LoginType {
    Username(String),
    Email(String),
    Phone(String),
}

#[derive(Debug)]
pub struct LoginRequest {
    login_type: LoginType,
    password: String,
}

impl<'de> Deserialize<'de> for LoginRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        // check if fields are present
        let login_type = match value.get("username") {
            Some(serde_json::Value::String(s)) => LoginType::Username(s.to_string()),
            _ => match value.get("email") {
                Some(serde_json::Value::String(s)) => LoginType::Email(s.to_string()),
                _ => match value.get("phone") {
                    Some(serde_json::Value::String(s)) => LoginType::Phone(s.to_string()),
                    _ => return Err(serde::de::Error::custom("Invalid login type")),
                },
            },
        };
        let password = match value.get("password") {
            Some(serde_json::Value::String(s)) => s.to_string(),
            _ => return Err(serde::de::Error::custom("Invalid password")),
        };
        Ok(Self {
            login_type,
            password,
        })
    }
}

#[debug_handler]
pub async fn login(
    State(ctx): State<AppContext>,
    Json(json): Json<LoginRequest>,
) -> Result<Response> {
    // Should probably use the Appropriate　HTTP status code

    let user = match &json.login_type {
        LoginType::Username(username) => user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to find user using username: {:?}", e);
                e
            })?,
        LoginType::Email(email) => user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to find user using email: {:?}", e);
                e
            })?,
        LoginType::Phone(phone) => user::Entity::find()
            .filter(user::Column::Phone.eq(phone))
            .one(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to find user using phone: {:?}", e);
                e
            })?,
    }
    .ok_or_else(|| {
        tracing::error!("Failed to find user");
        Error::NotFound
    })?;

    if user.password != json.password {
        return format::json(json!({
            "error": "Invalid password",
        }));
    }

    let jwt_config = ctx.config.get_jwt_config().map_err(|e| {
        tracing::error!("JWT config not found: {:?}", e);
        Error::InternalServerError
    })?;

    let time: i64 = std::time::SystemTime::now()
        .add(std::time::Duration::from_secs(jwt_config.expiration))
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| {
            tracing::error!("Failed to get time: {:?}", e);
            Error::InternalServerError
        })?
        .as_secs()
        .try_into()
        .map_err(|e| {
            tracing::error!("Failed to convert time: {:?}", e);
            Error::InternalServerError
        })?;

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &UserClaims {
            user_id: user.id,
            exp: time,
        },
        &jsonwebtoken::EncodingKey::from_secret(jwt_config.secret.as_ref()),
    )
    .map_err(|e| {
        tracing::error!("Failed to encode token: {:?}", e);
        Error::InternalServerError
    })?;

    format::json(json!({
        "token": token,
        "user_id": user.id,
        "exp": time,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth/")
        .add("register", post(register))
}
