use std::sync::Arc;

use anyhow::{anyhow, Context};
use log::info;
use serde::Deserialize;
use validator_derive::Validate;

use crate::article::application::article_repository;
use crate::article::domain::article::ArticleWithFavorite;
use crate::config;
use crate::config::app_state::ArcAppState;

pub async fn update_article(
    user_id: i64,
    slug: String,
    request: UpdateArticle,
    app_state: ArcAppState,
) -> config::Result<ArticleWithFavorite> {
    let article =
        article_repository::get_single_article_by_repository(slug.clone(), Arc::clone(&app_state))
            .await
            .context(format!("Don't have this slug article {}", slug))?;

    if article.is_not_author(user_id) {
        anyhow!("User is not author. Can't update article.");
    };

    let article = article
        .modify_title_option(request.title)
        .modify_body_option(request.body)
        .modify_description_option(request.description);

    let article = article_repository::update_article(article, Arc::clone(&app_state)).await?;

    info!("Success update article article id {}", article.id());

    let article = ArticleWithFavorite::new(article, false);

    Ok(article)
}

#[derive(Deserialize, Debug)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateArticleSlug {
    slug: String,
}
