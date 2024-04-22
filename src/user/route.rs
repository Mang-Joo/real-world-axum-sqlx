use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, post, put};

use crate::app_state::AppState;
use crate::error::error_handler;
use crate::user::application::handler::{get_current_user, login_user, registration_user, update_user_handler};

pub async fn user(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(registration_user))
        .route("/users", get(get_current_user))
        .route("/users", put(update_user_handler))
        .layer(Extension(error_handler))
        .layer(Extension(app_state))
}