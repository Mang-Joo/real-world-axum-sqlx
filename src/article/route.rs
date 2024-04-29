use axum::Router;
use axum::routing::{get, post};
use crate::article::handler::{create_article_handler, get_single_article_handler};

pub async fn article_route() -> Router {
    Router::new()
        .route("/articles", post(create_article_handler))
        .route("/articles/:slug", get(get_single_article_handler))
}