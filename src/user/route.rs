use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, post, put};

use crate::app_state::AppState;
use crate::error::error_handler;
use crate::user::application::follow_handler::{follow_user_handler, get_profile_handler};
use crate::user::application::user_handler::{get_current_user, login_user, registration_user, update_user_handler};

pub async fn user(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(registration_user))
        .route("/users", get(get_current_user))
        .route("/users", put(update_user_handler))
}

pub async fn follow(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/profiles/:username", get(get_profile_handler))
        .route("/profiles/:username/follow", post(follow_user_handler))
}

pub async fn route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", follow(app_state.clone()).await)
        .nest("/api", user(app_state.clone()).await)
        .layer(Extension(error_handler))
        .layer(Extension(app_state))
}