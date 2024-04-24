use std::sync::Arc;
use anyhow::anyhow;

use user::application::get_current_user_usecase;

use crate::{config, user};
use crate::article::domain::article::Article;
use crate::article::domain::tag::Tag;
use crate::config::app_state::AppState;
use crate::user::domain::user::User;

pub async fn create_article(
    user_id: i64,
    article: PostArticleRequest,
    app_state: Arc<AppState>,
) -> config::Result<Article> {
    let user = get_current_user_usecase::get_current_user(user_id, app_state)
        .await?;

    let article = article.to_domain(user);

    todo!()
}

pub struct PostArticleRequest {
    title: String,
    description: String,
    body: String,
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

        Article::new(
            0,
            self.title,
            self.description,
            self.body,
            tags,
            user,
        )
    }
}