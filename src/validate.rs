use axum::{async_trait, Json};
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
#[must_use]
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
            Ok(value) => { value }
            Err(_) => { return Err(AppError::Forbidden); }
        };


        if let Err(errors) = value.validate() {
            return Err(AppError::ValidateError(errors));
        }

        Ok(ValidationExtractor(value))
    }
}