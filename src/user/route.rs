use std::sync::Arc;

use axum::{middleware, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;

use crate::app_state::AppState;
use crate::user::handler::login_user;

pub async fn login(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .with_state(app_state)
        .layer(middleware::from_extractor())
}

async fn handle_error(err: anyhow::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}