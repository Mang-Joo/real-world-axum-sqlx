use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use axum::{async_trait, Json};
use log::{error, info};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::auth::jwt_decoder::JwtDecoder;
use crate::config::app_state::AppState;
use crate::config::error::AppError;

const AUTHORIZATION: &str = "Authorization";

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let result = Json::<T>::from_request(req, state).await;
        let Json(value) = match result {
            Ok(value) => value,
            Err(err) => {
                let msg = err.body_text();
                error!("Json parsing error {}", msg);
                if msg.contains("missing field") {
                    if let Some(field_name) = msg.split("missing field ").nth(1) {
                        let field_name = field_name
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .trim_matches('`')
                            .to_string();
                        return Err(AppError::MissingFieldError(field_name));
                    }
                };
                error!("{}", err);
                return Err(AppError::InternalServerError);
            }
        };

        if let Err(errors) = value.validate() {
            error!("Validation Error {}", errors);
            return Err(AppError::ValidateError(errors));
        }

        Ok(ValidationExtractor(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JwtValidationExtractor(pub i64);

#[async_trait]
impl<S> FromRequestParts<S> for JwtValidationExtractor
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Some(header) = parts.headers.get(AUTHORIZATION) {
            header
                .to_str()
                .map_err(|_| anyhow!("Invalid header value"))?
                .replace("Bearer ", "")
        } else {
            return Err(AppError::Unauthorized);
        };

        let app_state = parts
            .extensions
            .get::<Arc<AppState>>()
            .ok_or_else(|| {
                return AppError::AnyHow(anyhow!("Can't get _state"));
            })
            .map_err(|err| {
                return err;
            })?;

        let jwt_decoder = JwtDecoder::new(app_state.secret_key.clone());
        let payload = jwt_decoder.decode_token(&token)?;

        Ok(JwtValidationExtractor(payload.id()))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OptionalAuthenticateExtractor(pub Option<i64>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthenticateExtractor
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Some(header) = parts.headers.get(AUTHORIZATION) {
            match header.to_str() {
                Ok(header_value) => header_value.replace("Bearer ", ""),
                Err(_) => {
                    return Ok(OptionalAuthenticateExtractor(None));
                }
            }
        } else {
            return Ok(OptionalAuthenticateExtractor(None));
        };

        let app_state = parts
            .extensions
            .get::<Arc<AppState>>()
            .ok_or_else(|| {
                return AppError::AnyHow(anyhow!("Can't get _state"));
            })
            .map_err(|err| {
                return err;
            })?;

        let jwt_decoder = JwtDecoder::new(app_state.secret_key.clone());
        let payload = jwt_decoder.decode_token(&token)?;

        Ok(OptionalAuthenticateExtractor(Some(payload.id())))
    }
}
