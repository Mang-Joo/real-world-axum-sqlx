use std::env;
use std::sync::Arc;

use dotenv::dotenv;

use crate::config::db::{init_db, DbPool};

pub type ArcAppState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub secret_key: String,
}

impl AppState {
    pub fn new(secret_key: String) -> Self {
        AppState { secret_key }
    }
}

pub async fn init_app_state() -> AppState {
    let secret_key = env::var("SECRET_KEY").expect("Get Secret Key Error");

    AppState::new(secret_key)
}
