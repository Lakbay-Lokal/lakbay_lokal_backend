use crate::models::auth::{BusClaims, UserClaims};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use jsonwebtoken::DecodingKey;
use loco_rs::config::JWT;
use loco_rs::prelude::*;

static ADMIN_AUTH_KEY: &str = "QLaHJDmPYGup8cCHp7mM8IFVokiprELrirDD678u4Z_7-DNNlXB_7zfKhl6jT_65c47com48zUqfz_UsjGCvRQ";

#[derive(Debug, Clone, Copy)]
pub enum AuthType {
    Admin,
    Authenticated(UserClaims),
    Expired(UserClaims),
    Unauthorized,
    BadRequest,
}

#[derive(Debug, Clone, Copy)]
pub enum UserAuthType {
    Admin,
    Authenticated(UserClaims),
    Expired(UserClaims),
    Unauthorized,
    BadRequest,
}


impl AuthType {
    pub fn validate(&self) -> Result<Option<UserClaims>> {
        match self {
            AuthType::Admin => Ok(None),
            AuthType::Authenticated(UserClaims) => Ok(Some(UserClaims.clone())),
            AuthType::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            AuthType::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            AuthType::BadRequest => Err(Error::Unauthorized("Malformed token".to_string())),
        }
    }

    pub fn validate_by_id(&self, user_id: i32) -> Result<()> {
        match self {
            AuthType::Admin => Ok(()),
            AuthType::Authenticated(UserClaims) => {
                if UserClaims.user_id == user_id {
                    Ok(())
                } else {
                    Err(Error::Unauthorized("Unauthorized".to_string()))
                }
            }
            AuthType::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            AuthType::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            AuthType::BadRequest => Err(Error::BadRequest("Malformed or Non-existent token".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BusAuthType {
    Admin,
    Authenticated(BusClaims),
    Expired(BusClaims),
    Unauthorized,
    BadRequest,
}

impl BusAuthType {
    pub fn validate(&self) -> Result<Option<BusClaims>> {
        match self {
            BusAuthType::Admin => Ok(None),
            BusAuthType::Authenticated(BusClaims) => Ok(Some(BusClaims.clone())),
            BusAuthType::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            BusAuthType::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            BusAuthType::BadRequest => Err(Error::Unauthorized("Malformed token".to_string())),
        }
    }

    pub fn validate_by_id(&self, bus_id: i32) -> Result<()> {
        match self {
            BusAuthType::Admin => Ok(()),
            BusAuthType::Authenticated(BusClaims) => {
                if BusClaims.bus_id == bus_id {
                    Ok(())
                } else {
                    Err(Error::Unauthorized("Unauthorized".to_string()))
                }
            }
            BusAuthType::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            BusAuthType::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            BusAuthType::BadRequest => Err(Error::BadRequest("Malformed or Non-existent token".to_string())),
        }
    }
}

pub async fn middleware_authentication(
    State(jwt_config): State<JWT>,
    mut request_body: Request<Body>,
    next: Next,
) -> Response {
    if let Some(origin) = request_body.headers().get("X-Client-Source").and_then(|t| t.to_str().ok()) {
        if origin != "lakbay_lokal_mobile" {
            tracing::warn!("Unauthorized origin: {}", origin);
            request_body.extensions_mut().insert(AuthType::Unauthorized);
            return next.run(request_body).await
        }
    } else {
        tracing::warn!("Has no origin");
        request_body.extensions_mut().insert(AuthType::BadRequest);
        return next.run(request_body).await
    }

    if let Some(token) = request_body
        .headers()
        .get("Authorization")
        .and_then(|t| t.to_str().ok())
    {
        let token = token.strip_prefix("Bearer ").unwrap_or(token).to_string();
        tracing::info!("Token: {}", token);

        if token == ADMIN_AUTH_KEY {
                    request_body.extensions_mut().insert(AuthType::Admin);
            } else {
                match jsonwebtoken::decode::<UserClaims>(
                    &token,
                    &DecodingKey::from_secret(jwt_config.secret.as_ref()),
                    &jsonwebtoken::Validation::default(),
                ) {
                    Ok(token) => {
                        let claim = if token.claims.exp < chrono::Utc::now().timestamp() {
                            AuthType::Expired(token.claims)
                        } else {
                            AuthType::Authenticated(token.claims)
                        };

                        tracing::info!("Claim: {:?}", claim);
                        request_body.extensions_mut().insert(claim);
                        }
                    Err(_) => {
                        request_body.extensions_mut().insert(AuthType::BadRequest);
                    }
                }
        }
    } else {
        request_body.extensions_mut().insert(AuthType::Unauthorized);
    }

    next.run(request_body).await
}

pub async fn middleware_authentication_bus(
    State(jwt_config): State<JWT>,
    mut request_body: Request<Body>,
    next: Next,
) -> Response {
    if let Some(origin) = request_body.headers().get("X-Client-Source").and_then(|t| t.to_str().ok()) {
        if origin != "lakbay_lokal_bus_handheld" {
            tracing::warn!("Unauthorized origin: {}", origin);
            request_body.extensions_mut().insert(BusAuthType::Unauthorized);
            return next.run(request_body).await
        }
    } else {
        tracing::warn!("Has no origin");
        request_body.extensions_mut().insert(BusAuthType::BadRequest);
        return next.run(request_body).await
    }

    if let Some(token) = request_body
        .headers()
        .get("Authorization")
        .and_then(|t| t.to_str().ok())
    {
        let token = token.strip_prefix("Bearer ").unwrap_or(token).to_string();
        tracing::info!("Token: {}", token);

        if token == ADMIN_AUTH_KEY {
            request_body.extensions_mut().insert(BusAuthType::Admin);
        } else {
            match jsonwebtoken::decode::<BusClaims>(
                &token,
                &DecodingKey::from_secret(jwt_config.secret.as_ref()),
                &jsonwebtoken::Validation::default(),
            ) {
                Ok(token) => {
                    let claim = if token.claims.exp < chrono::Utc::now().timestamp() {
                        BusAuthType::Expired(token.claims)
                    } else {
                        BusAuthType::Authenticated(token.claims)
                    };

                    tracing::info!("Claim: {:?}", claim);
                    request_body.extensions_mut().insert(claim);
                }
                Err(_) => {
                    request_body.extensions_mut().insert(BusAuthType::BadRequest);
                }
            }
        }
    } else {
        request_body.extensions_mut().insert(BusAuthType::Unauthorized);
    }

    next.run(request_body).await
}