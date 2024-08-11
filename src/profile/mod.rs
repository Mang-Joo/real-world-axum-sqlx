use api::handler::get_profile;
use axum::{routing::get, Router};

pub mod api;
pub mod domain;
pub mod repository;
pub mod service;

pub fn profile_route() -> Router {
    Router::new().route("/profiles/:username", get(get_profile))
}
