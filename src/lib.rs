use std::env;
use std::sync::Arc;

use axum::{Extension, Router};
use config::db::{init_db, DbPool};
use config::di_factory::{create_profile_service, create_user_service};
use dotenv::dotenv;
use profile::profile_route;
use tokio::net::TcpListener;
use user::user_route;

use crate::config::app_state::{init_app_state, ArcAppState};
use crate::config::error::error_handler;

pub async fn start_application() -> () {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Failed get DB URL");
    let db_pool = init_db(database_url.as_ref()).await;

    let app_state = init_app_state().await;
    let app_state = Arc::new(app_state);

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let route = create_route(db_pool, app_state).await;

    axum::serve(listener, route.into_make_service())
        .await
        .unwrap();
}

pub async fn create_route(db_pool: DbPool, app_state: ArcAppState) -> Router {
    let user_service = create_user_service(db_pool.clone(), app_state.clone());
    let profile_service = create_profile_service(db_pool.clone(), user_service.clone());
    Router::new()
        .nest("/api", user_route())
        .nest("/api", profile_route())
        .layer(Extension(error_handler))
        .layer(Extension(app_state.clone()))
        .layer(Extension(user_service))
        .layer(Extension(profile_service))
}
pub mod auth;
pub mod config;
pub mod profile;
pub mod user;
