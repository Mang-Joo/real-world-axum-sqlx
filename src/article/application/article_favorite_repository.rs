use std::sync::Arc;

use anyhow::Context;

use crate::config;
use crate::config::app_state::AppState;

pub async fn is_favorite_article(
    user_id: i64,
    article_id: i64,
    app_state: Arc<AppState>,
) -> config::Result<bool> {
    let result = sqlx::query(
        r#"
    SELECT 1
    FROM article_favorite
    WHERE article_id = $1
    AND favorite_user_id = $2
    "#,
    )
    .bind(article_id)
    .bind(user_id)
    .fetch_optional(&app_state.pool)
    .await
    .context("Failed select is favorite")?;

    match result {
        None => Ok(false),
        Some(_) => Ok(true),
    }
}
