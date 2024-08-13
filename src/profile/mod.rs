use api::handler::{follow_user_api, get_profile, unfollow_api};
use axum::{
    routing::{delete, get, post},
    Router,
};

pub mod api;
pub mod domain;
pub mod repository;
pub mod service;

pub fn profile_route() -> Router {
    Router::new()
        .route("/profiles/:username", get(get_profile))
        .route("/profiles/:username/follow", post(follow_user_api))
        .route("/profiles/:username/unfollow", delete(unfollow_api))
}
