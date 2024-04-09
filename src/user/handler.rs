use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;

use crate::app_state::AppState;
use crate::user::application::login_usecase;
use crate::user::application::login_usecase::LoginRequest;

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> anyhow::Result<impl IntoResponse> {
    let response = login_usecase::login(state, payload)
        .await?;

    Ok(Json(response))
}