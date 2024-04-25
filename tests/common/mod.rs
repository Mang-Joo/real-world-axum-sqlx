use std::sync::Arc;

use axum::Router;

use real_world_axum_sqlx::config::app_state::init_app_state;
use real_world_axum_sqlx::user::route::user_route;

pub const TOKEN_FIXTURE: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6NSwic3ViIjoid2p3YW4wOTE2QGdtYWlsLmNvbSIsImV4cCI6MTc0NTQ1NzU2NCwiaWF0IjoxNzEzOTIxNTY0fQ.VdGRZ9g-GLbrDJRJVmHoY-Cm2ZYFgstj2AeAXmvh4pI";

pub async fn fixture_route() -> Router {
    user_route().await
}