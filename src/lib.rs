use std::sync::Arc;

use tokio::net::TcpListener;

use crate::config::app_state::init_app_state;
use crate::user::route::route;

pub mod article;
pub mod user;
pub mod auth;
pub mod config;

pub async fn start_application() -> () {
    let app_state = init_app_state().await;
    let app_state = Arc::new(app_state);

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let route = route(app_state).await;

    axum::serve(listener, route).await.unwrap();
}