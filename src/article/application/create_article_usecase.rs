use std::sync::Arc;

use log::info;
use serde::Deserialize;
use validator_derive::Validate;

use user::application::get_current_user_usecase;

use crate::{config, user};
use crate::article::application::article_repository;
use crate::article::domain::article::{Article, Author};
use crate::article::domain::tag::Tag;
use crate::config::app_state::AppState;
use crate::user::domain::user::User;

pub async fn create_article(
    user_id: i64,
    article_request: PostArticleRequest,
    app_state: Arc<AppState>,
) -> config::Result<Article> {
    let user = get_current_user_usecase::get_current_user(user_id, app_state.clone())
        .await?;

    let article = article_request.to_domain(user);

    let article = article_repository::save_article(article, &app_state.pool)
        .await?;

    info!("Create article succeed");

    Ok(article)
}

#[derive(Deserialize, Validate)]
pub struct PostArticleRequest {
    #[validate(required(message = "article title is required."))]
    title: Option<String>,
    #[validate(required(message = "article description is required."))]
    description: Option<String>,
    #[validate(required(message = "article content is required."))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Option<Vec<String>>,
}

impl PostArticleRequest {
    fn to_domain(self, user: User) -> Article {
        let tags = if let Some(tags) = self.tag_list {
            let tag_list = tags.into_iter()
                .map(|tag| Tag::new(tag))
                .collect::<Vec<Tag>>();
            Some(tag_list)
        } else { None };

        let author = Author::new(
            user.id(),
            user.user_name().to_owned(),
            user.bio().to_owned(),
            user.image().to_owned(),
        );

        Article::new(
            0,
            self.title.unwrap(),
            self.description.unwrap(),
            self.body.unwrap(),
            0,
            tags,
            author,
        )
    }
}