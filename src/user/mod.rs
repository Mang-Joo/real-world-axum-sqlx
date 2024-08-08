use axum::{routing::post, Router};
use user_handler::{login_api, register_api};

pub mod domain;
pub mod repository;
pub mod service;
pub mod user_handler;

pub fn user_route() -> Router {
    Router::new()
        .route("/users", post(register_api))
        .route("/users/login", post(login_api))
}
