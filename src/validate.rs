use std::sync::Arc;

use anyhow::anyhow;
use axum::{async_trait, Json};
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app_state::AppState;
use crate::auth::jwt_decoder::JwtDecoder;
use crate::error::AppError;

const AUTHORIZATION: &str = "Authorization";

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationExtractor<T>(pub T);


#[async_trait]
impl<T, S> FromRequest<S> for ValidationExtractor<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync, {
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let result = Json::<T>::from_request(req, state).await;
        let Json(value) = match result {
            Ok(value) => { value }
            Err(_) => { return Err(AppError::Forbidden); }
        };


        if let Err(errors) = value.validate() {
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
            header.to_str()
                .map_err(|_| anyhow!("Invalid header value"))?
                .replace("Bearer ", "")
        } else {
            return Err(AppError::Unauthorized);
        };


        let app_state = get_app_state(parts);

        let jwt_decoder = JwtDecoder::new(app_state.to_owned());
        let payload = jwt_decoder.decode_token(&token).await?;

        Ok(JwtValidationExtractor(payload.id()))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OptionalAuthenticateExtractor(pub Option<i64>);

impl<S> FromRequestParts<S> for OptionalAuthenticateExtractor
    where
        S: Send + Sync {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Some(header) = parts.headers.get(AUTHORIZATION) {
            header.to_str()
                .replace("Bearer ", "")
        } else {
            return Ok(None);
        };


        let app_state = Self::get_app_state(parts)?;

        let jwt_decoder = JwtDecoder::new(app_state.to_owned());
        let payload = jwt_decoder.decode_token(&token).await?;

        Ok(JwtValidationExtractor(payload.id()))
    }
}

fn get_app_state(parts: &mut Parts) -> Result<&Arc<AppState>, AppError> {
    let app_state = parts.extensions.get::<Arc<AppState>>().ok_or_else(|| {
        return AppError::AnyHow(anyhow!("Can't get _state"));
    })
        .map_err(|err| { return err; })?;

    Ok(app_state)
}