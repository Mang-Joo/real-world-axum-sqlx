use std::time::Duration;
use sqlx::MySql;
use sqlx::mysql::MySqlPoolOptions;
use crate::app_state::Config;

pub type DbPool = sqlx::Pool<MySql>;

pub async fn init_db(config: &Config) -> DbPool {
    let db_url = config.database_url();

    MySqlPoolOptions::new()
        .max_connections(30)
        .idle_timeout(Duration::from_secs(3))
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect(db_url.as_ref())
        .await
        .unwrap()
}