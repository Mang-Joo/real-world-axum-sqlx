use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_macros::debug_handler;

use crate::app_state::AppState;
use crate::user::application::login_usecase;
use crate::user::application::login_usecase::LoginRequest;

#[debug_handler]
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {

    let response = login_usecase::login(state, payload)
        .await
        .map_err(|err| return StatusCode::BAD_REQUEST)
        .unwrap();

    Ok(Json(response))
}