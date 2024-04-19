use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, post};

use crate::app_state::AppState;
use crate::error::error_handler;
use crate::user::application::handler::{get_current_user, login_user, registration_user};

pub async fn user(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(registration_user))
        .route("/users", get(get_current_user))
        .layer(Extension(error_handler))
        .layer(Extension(app_state.clone()))
        .with_state(app_state)
}