use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use serde::Deserialize;
use serde_json::Value;

use real_world_axum_sqlx::config::app_state::init_app_state;
use real_world_axum_sqlx::create_route;

pub const TOKEN_FIXTURE: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwic3ViIjoid2p3YW4wOTE1QGdtYWlsLmNvbSIsImV4cCI6MTc0NjU5ODYwOCwiaWF0IjoxNzE1MDYyNjA4fQ.AvdXtrVkm7jYk0xi4RTdrT9n0d3j1SgJ-V-i5KsNQ80";

pub async fn fixture_route() -> Router {
    let app_state = Arc::new(init_app_state().await);
    create_route(app_state).await
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    #[serde(flatten)]
    pub data: HashMap<String, Option<Value>>,
}