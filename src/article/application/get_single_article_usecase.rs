use std::sync::Arc;

use crate::article::application::article_favorite_repository::is_favorite;
use crate::article::application::article_repository::get_single_article_by_repository;
use crate::article::domain::article::ArticleWithFavorite;
use crate::config;
use crate::config::app_state::AppState;

pub async fn get_single_article(
    user_id: Option<i64>,
    slug: String,
    app_state: Arc<AppState>,
) -> config::Result<ArticleWithFavorite> {
    let db_pool = &app_state.pool;

    let article = get_single_article_by_repository(
        slug,
        db_pool,
    ).await?;

    let is_favorite = match user_id {
        None => { false }
        Some(user_id) => {
            let is_favorite = is_favorite(
                article.id(),
                user_id,
                db_pool,
            ).await?;

            is_favorite
        }
    };

    let article_with_favorite = ArticleWithFavorite::new(article, is_favorite);

    Ok(article_with_favorite)
}

