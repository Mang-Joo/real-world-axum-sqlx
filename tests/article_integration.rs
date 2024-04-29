use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use crate::common::fixture_route;

mod common;

#[tokio::test]
pub async fn save_article_200_Ok() {
    let request_body = json!({
        "title": "Hello mangjoo ",
        "description": "Test Description",
        "body": "content body",
        "tagList": ["mangjoo", "test", "test"]
    }).to_string();

    let request = Request::builder()
        .uri("/api/articles")
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", common::TOKEN_FIXTURE)
        .body(Body::from(request_body))
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
pub async fn save_article_400_bad_request() {
    let request_body = json!({
        "title": null,
        "description": null,
        "body": null,
        "tagList": ["mangjoo", "test", "test"]
    }).to_string();

    let request = Request::builder()
        .uri("/api/articles")
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", common::TOKEN_FIXTURE)
        .body(Body::from(request_body))
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
