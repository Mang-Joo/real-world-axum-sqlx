use std::env;

use dotenv::dotenv;

use crate::db::{DbPool, init_db};
// use crate::error::AppError;


#[derive(Debug)]
pub struct AppState {
    pub pool: DbPool,
    pub secret_key: String,
}

impl AppState {
    pub fn new(db_pool: DbPool, secret_key: String) -> Self {
        AppState {
            pool: db_pool,
            secret_key,
        }
    }
}

pub async fn init_app_state() -> AppState {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Get DB Error");

    let secret_key = env::var("SECRET_KEY")
        .expect("Get Secret Key Error");

    let db = init_db(database_url).await;
    AppState::new(db, secret_key)
}