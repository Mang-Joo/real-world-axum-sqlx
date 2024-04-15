use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::user::application::login_usecase;
use crate::user::application::login_usecase::LoginRequest;

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = login_usecase::user_login(state, payload)
        .await
        .map_err(|err| return AppError::AnyHow(err))?;

    Ok(Json(response))
}