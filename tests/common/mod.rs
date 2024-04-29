use std::sync::Arc;

use axum::Router;

use real_world_axum_sqlx::config::app_state::init_app_state;
use real_world_axum_sqlx::create_route;

pub const TOKEN_FIXTURE: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NSwic3ViIjoid2p3YW4wOTE2QGdtYWlsLmNvbSIsImV4cCI6MTcxNDYxMDk2MiwiaWF0IjoxNzE0MzUxNzYyfQ._tIcuy5anTm3VifuB4flj9i8lUu4cL0Qxs_CbxN0Wrs";

pub async fn fixture_route() -> Router {
    let app_state = Arc::new(init_app_state().await);
    create_route(app_state).await
}