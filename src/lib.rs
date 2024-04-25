use std::sync::Arc;

use axum::{Extension, Router};
use tokio::net::TcpListener;

use crate::article::route::article_route;
use crate::config::app_state::{AppState, init_app_state};
use crate::config::error::error_handler;
use crate::user::route::user_route;

pub mod article;
pub mod user;
pub mod auth;
pub mod config;

pub async fn start_application() -> () {
    let app_state = init_app_state().await;
    let app_state = Arc::new(app_state);

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let route = create_route(app_state).await;

    axum::serve(listener, route).await.unwrap();
}

async fn create_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", user_route().await)
        .nest("/api", article_route().await)
        .layer(Extension(error_handler))
        .layer(Extension(app_state))
}