use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub type DbPool = sqlx::Pool<Postgres>;

pub async fn init_db(database_url: &str) -> DbPool {
    PgPoolOptions::new()
        .connect(database_url)
        .await
        .context("Db Init Error")
        .unwrap()
}
