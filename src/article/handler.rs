use std::sync::Arc;

use axum::{Extension, Json};
use axum::response::IntoResponse;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::article::application::create_article_usecase::{create_article, PostArticleRequest};
use crate::article::domain::article::Article;
use crate::config::app_state::AppState;
use crate::config::error::AppError;
use crate::config::validate::{JwtValidationExtractor, ValidationExtractor};
use crate::user::application::get_current_user_usecase::get_current_user;
use crate::user::domain::user::User;

pub async fn create_article_handler(
    JwtValidationExtractor(user_id): JwtValidationExtractor,
    Extension(state): Extension<Arc<AppState>>,
    ValidationExtractor(request): ValidationExtractor<PostArticleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let created_article = create_article(user_id, request, state.clone()).await?;
    let user = get_current_user(user_id, state).await?;

    let response = ArticleResponse::from_domain(created_article, user);

    Ok(Json(response))
}

#[derive(Serialize)]
struct ArticleResponse {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    favorite: bool,
    favorite_count: i64,
    author: AuthorResponse,
}

impl ArticleResponse {
    fn from_domain(
        article: Article,
        user: User,
    ) -> Self {
        let tags = if let Some(tags) = article.tag_list() {
            let tags = tags
                .into_iter()
                .map(|tag| tag.tag().to_owned())
                .collect::<Vec<String>>();
            Some(tags)
        } else { None };

        ArticleResponse {
            slug: article.slug().to_owned(),
            title: article.title().to_owned(),
            description: article.description().to_owned(),
            body: article.body().to_owned(),
            tag_list: tags,
            created_at: article.created_at().naive_utc(),
            updated_at: article.updated_at().naive_utc(),
            favorite: false,
            favorite_count: 0,
            author: AuthorResponse::from_user(user),
        }
    }
}

#[derive(Serialize)]
struct AuthorResponse {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl AuthorResponse {
    fn from_user(user: User) -> Self {
        AuthorResponse {
            username: user.user_name().to_owned(),
            bio: user.bio().to_owned(),
            image: user.image().to_owned(),
            following: false,
        }
    }
}