use std::time::Duration;

use sqlx::{Postgres};
use sqlx::postgres::PgPoolOptions;

pub type DbPool = sqlx::Pool<Postgres>;

pub async fn init_db(database_url: String) -> DbPool {
    PgPoolOptions::new()
        .max_connections(30)
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect(database_url.as_ref())
        .await
        .unwrap()
}