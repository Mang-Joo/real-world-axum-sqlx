use std::sync::Arc;

use anyhow::anyhow;
use axum::{async_trait, Json};
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app_state::AppState;
use crate::auth::jwt_decoder::JwtDecoder;
use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
#[must_use]
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

pub struct JwtValidationExtractor(pub i64);

#[async_trait]
impl<S> FromRequest<S> for JwtValidationExtractor
    where
        S: Send + Sync, {
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Some(header) = req.headers().get("Authorization") {
            header.to_str()
                .map_err(|_| anyhow!("Invalid header value"))?
                .replace("Bearer ", "")
        } else {
            return Err(AppError::Unauthorized);
        };

        let app_state = req.extensions().get::<Arc<AppState>>().ok_or_else(|| {
            return AppError::AnyHow(anyhow!("Can't get state"));
        })
            .map_err(|err| { return err; })?;

        let jwt_decoder = JwtDecoder::new(app_state.to_owned());
        let payload = jwt_decoder.decode_token(&token).await?;

        Ok(JwtValidationExtractor(payload.id()))
    }
}