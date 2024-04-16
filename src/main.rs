use std::sync::Arc;

use tokio::net::TcpListener;

use crate::app_state::init_app_state;
use crate::user::route::login;

mod app_state;
mod db;
mod auth;
mod user;
mod error;
mod validate;


#[tokio::main]
async fn main() {
    let app_state = init_app_state().await;
    let app_state = Arc::new(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let route = login(app_state).await;

    axum::serve(listener, route).await.unwrap();
}
