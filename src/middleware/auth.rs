#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::auth::{BusClaims, UserClaims};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use jsonwebtoken::DecodingKey;
use loco_rs::config::JWT;
use loco_rs::prelude::*;

static ADMIN_AUTH_KEY: &str =
    "QLaHJDmPYGup8cCHp7mM8IFVokiprELrirDD678u4Z_7-DNNlXB_7zfKhl6jT_65c47com48zUqfz_UsjGCvRQ";

#[derive(Debug, Clone, Copy)]
pub enum AuthorizationType {
    Admin,
    Authenticated(UserClaims),
    Expired(UserClaims),
    Unauthorized,
    BadRequest,
}

impl AuthorizationType {
    pub fn validate(&self) -> Result<Option<UserClaims>> {
        match self {
            Self::Admin => Ok(None),
            Self::Authenticated(user_claims) => Ok(Some(*user_claims)),
            Self::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            Self::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            Self::BadRequest => Err(Error::Unauthorized("Malformed token".to_string())),
        }
    }

    pub fn validate_by_id(&self, user_id: i32) -> Result<()> {
        match self {
            Self::Admin => Ok(()),
            Self::Authenticated(user_claims) => {
                if user_claims.user_id == user_id {
                    Ok(())
                } else {
                    Err(Error::Unauthorized("Unauthorized".to_string()))
                }
            }
            Self::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            Self::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            Self::BadRequest => Err(Error::BadRequest(
                "Malformed or Non-existent token".to_string(),
            )),
        }
    }

    pub fn admin_only(&self) -> Result<()> {
        match self {
            Self::Admin => Ok(()),
            _ => Err(Error::Unauthorized("Unauthorized".to_string())),
        }
    }

    pub fn validate_with(
        &self,
        other: &BusAuthorizationType,
    ) -> Result<(Option<UserClaims>, Option<BusClaims>)> {
        let bus_auth = other.validate();
        let user_auth = self.validate();

        match (user_auth, bus_auth) {
            (Ok(Some(user_claims)), Ok(Some(bus_claims))) => {
                Ok((Some(user_claims), Some(bus_claims)))
            }
            (Ok(Some(user_claims)), Ok(None)) => Ok((Some(user_claims), None)),
            (Ok(None), Ok(Some(bus_claims))) => Ok((None, Some(bus_claims))),
            _ => Err(Error::Unauthorized("Unauthorized".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BusAuthorizationType {
    Admin,
    Authenticated(BusClaims),
    Expired(BusClaims),
    Unauthorized,
    BadRequest,
}

impl BusAuthorizationType {
    pub fn validate(&self) -> Result<Option<BusClaims>> {
        match self {
            Self::Admin => Ok(None),
            Self::Authenticated(bus_claims) => Ok(Some(*bus_claims)),
            Self::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            Self::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            Self::BadRequest => Err(Error::Unauthorized("Malformed token".to_string())),
        }
    }

    pub fn validate_by_id(&self, bus_id: i32) -> Result<()> {
        match self {
            Self::Admin => Ok(()),
            Self::Authenticated(bus_claims) => {
                if bus_claims.bus_id == bus_id {
                    Ok(())
                } else {
                    Err(Error::Unauthorized("Unauthorized".to_string()))
                }
            }
            Self::Expired(_) => Err(Error::Unauthorized("Token expired".to_string())),
            Self::Unauthorized => Err(Error::Unauthorized("Unauthorized".to_string())),
            Self::BadRequest => Err(Error::BadRequest(
                "Malformed or Non-existent token".to_string(),
            )),
        }
    }

    pub fn admin_only(&self) -> Result<()> {
        match self {
            Self::Admin => Ok(()),
            _ => Err(Error::Unauthorized("Unauthorized".to_string())),
        }
    }

    pub fn validate_with(
        &self,
        other: &AuthorizationType,
    ) -> Result<(Option<UserClaims>, Option<BusClaims>)> {
        other.validate_with(self)
    }
}

pub async fn middleware_authentication(
    State(jwt_config): State<JWT>,
    mut request_body: Request<Body>,
    next: Next,
) -> Response {
    if let Some(origin) = request_body
        .headers()
        .get("X-Client-Source")
        .and_then(|t| t.to_str().ok())
    {
        if origin != "lakbay_lokal_mobile" {
            tracing::warn!("Unauthorized origin: {}", origin);
            request_body
                .extensions_mut()
                .insert(AuthorizationType::Unauthorized);
            return next.run(request_body).await;
        }
    } else {
        tracing::warn!("Has no origin");
        request_body
            .extensions_mut()
            .insert(AuthorizationType::BadRequest);
        return next.run(request_body).await;
    }

    if let Some(token) = request_body
        .headers()
        .get("Authorization")
        .and_then(|t| t.to_str().ok())
    {
        let token = token.strip_prefix("Bearer ").unwrap_or(token).to_string();
        tracing::info!("Token: {}", token);

        if token == ADMIN_AUTH_KEY {
            request_body
                .extensions_mut()
                .insert(AuthorizationType::Admin);
        } else {
            match jsonwebtoken::decode::<UserClaims>(
                &token,
                &DecodingKey::from_secret(jwt_config.secret.as_ref()),
                &jsonwebtoken::Validation::default(),
            ) {
                Ok(token) => {
                    let claim = if token.claims.exp < chrono::Utc::now().timestamp() {
                        AuthorizationType::Expired(token.claims)
                    } else {
                        AuthorizationType::Authenticated(token.claims)
                    };

                    tracing::info!("Claim: {:?}", claim);
                    request_body.extensions_mut().insert(claim);
                }
                Err(_) => {
                    request_body
                        .extensions_mut()
                        .insert(AuthorizationType::BadRequest);
                }
            }
        }
    } else {
        request_body
            .extensions_mut()
            .insert(AuthorizationType::Unauthorized);
    }

    next.run(request_body).await
}

pub async fn middleware_authentication_bus(
    State(jwt_config): State<JWT>,
    mut request_body: Request<Body>,
    next: Next,
) -> Response {
    if let Some(origin) = request_body
        .headers()
        .get("X-Client-Source")
        .and_then(|t| t.to_str().ok())
    {
        if origin != "lakbay_lokal_bus_handheld" {
            tracing::warn!("Unauthorized origin: {}", origin);
            request_body
                .extensions_mut()
                .insert(BusAuthorizationType::Unauthorized);
            return next.run(request_body).await;
        }
    } else {
        tracing::warn!("Has no origin");
        request_body
            .extensions_mut()
            .insert(BusAuthorizationType::BadRequest);
        return next.run(request_body).await;
    }

    if let Some(token) = request_body
        .headers()
        .get("Authorization")
        .and_then(|t| t.to_str().ok())
    {
        let token = token.strip_prefix("Bearer ").unwrap_or(token).to_string();
        tracing::info!("Token: {}", token);

        if token == ADMIN_AUTH_KEY {
            request_body
                .extensions_mut()
                .insert(BusAuthorizationType::Admin);
        } else {
            match jsonwebtoken::decode::<BusClaims>(
                &token,
                &DecodingKey::from_secret(jwt_config.secret.as_ref()),
                &jsonwebtoken::Validation::default(),
            ) {
                Ok(token) => {
                    let claim = if token.claims.exp < chrono::Utc::now().timestamp() {
                        BusAuthorizationType::Expired(token.claims)
                    } else {
                        BusAuthorizationType::Authenticated(token.claims)
                    };

                    tracing::info!("Claim: {:?}", claim);
                    request_body.extensions_mut().insert(claim);
                }
                Err(_) => {
                    request_body
                        .extensions_mut()
                        .insert(BusAuthorizationType::BadRequest);
                }
            }
        }
    } else {
        request_body
            .extensions_mut()
            .insert(BusAuthorizationType::Unauthorized);
    }

    next.run(request_body).await
}
