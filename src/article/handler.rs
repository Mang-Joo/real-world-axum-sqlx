use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::article::application::create_article_usecase::{create_article, PostArticleRequest};
use crate::article::application::get_articles_default_usecase::{
    get_article_default, ListArticleRequest,
};
use crate::article::application::get_single_article_usecase::get_single_article;
use crate::article::domain::article::{Article, Author};
use crate::config::app_state::AppState;
use crate::config::error::AppError;
use crate::config::validate::{
    JwtValidationExtractor, OptionalAuthenticateExtractor, ValidationExtractor,
};

pub async fn create_article_handler(
    JwtValidationExtractor(user_id): JwtValidationExtractor,
    Extension(state): Extension<Arc<AppState>>,
    ValidationExtractor(request): ValidationExtractor<PostArticleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let created_article = create_article(user_id, request, Arc::clone(&state)).await?;

    let response = ArticleResponse::from_domain(created_article, false);

    Ok(Json(response))
}

pub async fn get_single_article_handler(
    OptionalAuthenticateExtractor(user_id): OptionalAuthenticateExtractor,
    Extension(state): Extension<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let article = get_single_article(user_id, slug, Arc::clone(&state)).await?;

    let response =
        ArticleResponse::from_domain(article.article().to_owned(), article.is_favorite());
    Ok(Json(response))
}

pub async fn get_default_articles_handler(
    OptionalAuthenticateExtractor(user_id): OptionalAuthenticateExtractor,
    Query(query): Query<ListArticleRequest>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let articles = get_article_default(user_id, query, Arc::clone(&state)).await?;

    let article_response = articles
        .into_iter()
        .map(|article| {
            let response =
                ArticleResponse::from_domain(article.article().to_owned(), article.is_favorite());
            response
        })
        .collect::<Vec<ArticleResponse>>();

    let response = MultipleArticleResponse::new(article_response);

    Ok(Json(response))
}

#[derive(Serialize)]
struct MultipleArticleResponse {
    articles: Vec<ArticleResponse>,
    #[serde(rename = "articlesCount")]
    articles_count: usize,
}

impl MultipleArticleResponse {
    pub fn new(articles: Vec<ArticleResponse>) -> Self {
        let articles_count = articles.len();

        Self {
            articles,
            articles_count,
        }
    }
}

#[derive(Serialize)]
struct ArticleResponse {
    slug: String,
    title: String,
    description: String,
    body: String,
    #[serde(rename = "tagList")]
    tag_list: Option<Vec<String>>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    favorite: bool,
    #[serde(rename = "favoriteCount")]
    favorite_count: i64,
    author: AuthorResponse,
}

impl ArticleResponse {
    fn from_domain(article: Article, is_favorite: bool) -> Self {
        let tags = if let Some(tags) = article.tag_list() {
            let tags = tags
                .into_iter()
                .map(|tag| tag.tag().to_owned())
                .collect::<Vec<String>>();
            Some(tags)
        } else {
            None
        };

        ArticleResponse {
            slug: article.slug().to_owned(),
            title: article.title().to_owned(),
            description: article.description().to_owned(),
            body: article.body().to_owned(),
            tag_list: tags,
            created_at: article.created_at().naive_utc(),
            updated_at: article.updated_at().naive_utc(),
            favorite: is_favorite,
            favorite_count: article.favorite_count(),
            author: AuthorResponse::from_user(article.author().to_owned()),
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
    fn from_user(author: Author) -> Self {
        AuthorResponse {
            username: author.user_name().to_owned(),
            bio: author.bio().to_owned(),
            image: author.image().to_owned(),
            following: false,
        }
    }
}
