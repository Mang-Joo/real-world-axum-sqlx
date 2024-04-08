use std::env;
use std::sync::Arc;
use axum::middleware::AddExtension;
use axum::{Extension, Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, Route};
use clap::Parser;
use dotenv::dotenv;
use sqlx::Executor;
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use crate::app_state::{AppState, Config};
use crate::db::init_db;

mod app_state;
mod db;


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
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let hmac_key = env::var("HMAC_KEY").unwrap();

    let config = Config {
        database_url,
        hmac_key,
    };

    let db = init_db(&config).await;

    let app_state = Arc::new(app_state::AppState::new(db));


    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let route = Router::new()
        .route("/", get(handler))
        .layer(Extension(app_state));

    axum::serve(listener, route).await.unwrap();
}
