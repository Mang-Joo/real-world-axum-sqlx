use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{delete, get, post, put};

use crate::config::app_state::AppState;
use crate::config::error::error_handler;
use crate::user::application::follow_handler::{follow_user_handler, get_profile_handler, unfollow_user_handler};
use crate::user::application::user_handler::{get_current_user, login_user, registration_user, update_user_handler};

pub async fn user() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/users", post(registration_user))
        .route("/users", get(get_current_user))
        .route("/users", put(update_user_handler))
}

pub async fn follow() -> Router {
    Router::new()
        .route("/profiles/:username", get(get_profile_handler))
        .route("/profiles/:username/follow", post(follow_user_handler))
        .route("/profiles/:username/follow", delete(unfollow_user_handler))
}

pub async fn route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", follow().await)
        .nest("/api", user().await)
        .layer(Extension(error_handler))
        .layer(Extension(app_state))
}