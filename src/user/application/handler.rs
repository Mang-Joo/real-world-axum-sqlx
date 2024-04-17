use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::user::application::{login_usecase, registration_usecase};
use crate::user::application::login_usecase::LoginRequest;
use crate::user::application::registration_usecase::RegistrationUserRequest;
use crate::validate::ValidationExtractor;

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    ValidationExtractor(payload): ValidationExtractor<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = login_usecase::user_login(state, payload)
        .await
        .map_err(|err| {
            return AppError::AnyHow(err);
        })?;

    Ok(Json(response))
}

pub async fn registration_user(
    State(state): State<Arc<AppState>>,
    ValidationExtractor(request): ValidationExtractor<RegistrationUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = registration_usecase::registration(
        state,
        request,
    ).await?;

    Ok(Json(response))
}