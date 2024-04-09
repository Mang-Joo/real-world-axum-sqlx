use std::time::Duration;

use sqlx::MySql;
use sqlx::mysql::MySqlPoolOptions;

pub type DbPool = sqlx::Pool<MySql>;

pub async fn init_db(database_url: String) -> DbPool {
    MySqlPoolOptions::new()
        .max_connections(30)
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect(database_url.as_ref())
        .await
        .unwrap()
}