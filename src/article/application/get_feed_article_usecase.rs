use std::sync::Arc;
use log::info;
use serde::Deserialize;
use crate::article::application::article_repository;
use crate::article::domain::article::ArticleWithFavorite;
use crate::config;
use crate::config::app_state::{AppState, ArcAppState};

pub async fn get_feed_articles(
    user_id: i64,
    request: FeedArticleRequest,
    app_state: ArcAppState
) -> config::Result<Vec<ArticleWithFavorite>> {
    let articles = article_repository::get_feed_articles_by_respository(
        user_id,
        request.limit.unwrap_or(20),
        request.offset.unwrap_or(0),
        Arc::clone(&app_state),
    ).await?;

    info!("Succeed get feed articles by user : {}", user_id);

    Ok(articles)
}

#[derive(Deserialize)]
pub struct FeedArticleRequest {
    limit: Option<i64>,
    offset: Option<i64>
}