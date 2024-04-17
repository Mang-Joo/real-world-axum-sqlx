use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::post;

use crate::app_state::AppState;
use crate::error::error_handler;
use crate::user::application::handler::{login_user, registration_user};

pub async fn login(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(registration_user))
        .layer(Extension(error_handler))
        .with_state(app_state)
}