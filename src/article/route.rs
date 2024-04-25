use axum::Router;
use axum::routing::post;
use crate::article::handler::create_article_handler;

pub async fn article_route() -> Router {
    Router::new()
        .route("/articles", post(create_article_handler))
}