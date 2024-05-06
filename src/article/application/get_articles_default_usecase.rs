use std::sync::Arc;

use serde::Deserialize;

use crate::article::application::article_repository::get_default_articles_by_repository;
use crate::article::domain::article::ArticleWithFavorite;
use crate::config;
use crate::config::app_state::AppState;

pub async fn get_article_default(
    user_id: Option<i64>,
    request: ListArticleRequest,
    app_state: Arc<AppState>,
) -> config::Result<Vec<ArticleWithFavorite>> {
    let articles =
        get_default_articles_by_repository(user_id, request, Arc::clone(&app_state)).await?;

    Ok(articles)
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArticleRequest {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl ListArticleRequest {
    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }
    pub fn author(&self) -> &Option<String> {
        &self.author
    }
    pub fn favorited(&self) -> &Option<String> {
        &self.favorited
    }
    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
    pub fn offset(&self) -> Option<usize> {
        self.offset
    }
}
