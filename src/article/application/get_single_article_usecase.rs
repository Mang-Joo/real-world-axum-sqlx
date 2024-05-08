use std::sync::Arc;

use crate::article::application::article_favorite_repository::is_favorite_article;
use crate::article::application::article_repository::get_single_article_by_repository;
use crate::article::domain::article::ArticleWithFavorite;
use crate::config;
use crate::config::app_state::ArcAppState;

pub async fn get_single_article(
    user_id: Option<i64>,
    slug: String,
    app_state: ArcAppState,
) -> config::Result<ArticleWithFavorite> {
    let article = get_single_article_by_repository(slug, Arc::clone(&app_state)).await?;

    let is_favorite = match user_id {
        None => false,
        Some(user_id) => {
            let is_favorite =
                is_favorite_article(article.id(), user_id, Arc::clone(&app_state)).await?;

            is_favorite
        }
    };

    let article_with_favorite = ArticleWithFavorite::new(article, is_favorite);

    Ok(article_with_favorite)
}
