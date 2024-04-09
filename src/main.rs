use std::sync::Arc;

use axum::{Extension, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::net::TcpListener;

use crate::app_state::{AppState, init_app_state};

mod app_state;
mod db;
mod auth;
mod user;
mod error;


async fn handler(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let query = r#"
        CREATE TABLE users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;
    sqlx::query(query)
        .execute(&state.pool)
        .await
        .ok();

    println!("hello {:?}", state);
    eprintln!("hello {:?}", state);
    "helloWorld".into_response()
}


#[tokio::main]
async fn main() {
    let app_state = init_app_state().await;
    let app_state = Arc::new(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let route = Router::new()
        .route("/", get(handler))
        .layer(Extension(app_state));

    axum::serve(listener, route).await.unwrap();
}
