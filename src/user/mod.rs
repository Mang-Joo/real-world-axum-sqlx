use axum::{
    routing::{get, post, put},
    Router,
};
use user_handler::{get_info_api, login_api, register_api, update_user_api};

pub mod domain;
pub mod repository;
pub mod service;
pub mod user_handler;

pub fn user_route() -> Router {
    Router::new()
        .route("/users", post(register_api))
        .route("/users/login", post(login_api))
        .route("/user", get(get_info_api))
        .route("/user", put(update_user_api))
}
