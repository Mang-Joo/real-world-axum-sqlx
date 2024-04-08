use crate::db::DbPool;

#[derive(Debug)]
pub struct AppState {
    pub pool: DbPool,
    pub secret_key: String,
}

impl AppState {
    pub fn new(db_pool: DbPool, secret_key: String) -> Self {
        AppState {
            pool: db_pool,
            secret_key
        }
    }
}