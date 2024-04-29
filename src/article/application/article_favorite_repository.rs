use anyhow::{anyhow, Context};
use log::{error, info};

use crate::config;
use crate::config::db::DbPool;

pub async fn count_favorite_by_article_id(
    article_id: i64,
    db_pool: &DbPool,
) -> config::Result<i64> {
    let count: i64 = sqlx::query_scalar("
        SELECT count(*)
        FROM article_favorite
        WHERE article_id = ?"
    )
        .bind(article_id)
        .fetch_one(db_pool)
        .await
        .map_err(|err| {
            error!("Failed get count by article {}", err.to_string());
            anyhow!("Failed get count by article")
        })?;

    info!("Succeed get count by article id : {article_id}");

    Ok(count)
}

pub async fn is_favorite(
    user_id: i64,
    article_id: i64,
    db_pool: &DbPool,
) -> config::Result<bool> {
    let result = sqlx::query("
    SELECT 1
    FROM article_favorite
    WHERE article_id = ?
    AND user_id = ?
    ")
        .bind(article_id)
        .bind(user_id)
        .fetch_optional(db_pool)
        .await
        .context("Failed select is favorite")?;

    match result {
        None => { Ok(false) }
        Some(_) => { Ok(true) }
    }
}