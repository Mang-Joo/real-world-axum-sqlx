use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use axum::response::Response;
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::common::{fixture_route, ResponseData};

mod common;

#[tokio::test]
pub async fn save_article_200_ok() {
    let request_body = json!({
        "title": "Hello mangjoo",
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

#[tokio::test]
async fn get_single_article_200_ok() {
    let slug = "Hello-mangjoo-";
    let request = Request::builder()
        .uri(format!("/api/articles/{}", slug))
        .method(Method::GET)
        .header("Content-Type", "application/json")
        // .header("Authorization", common::TOKEN_FIXTURE)
        .body(Body::empty())
        .unwrap();

    let response: Response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    let byte = to_bytes(response.into_body(), usize::MAX).await
        .expect("");

    let result: ResponseData = serde_json::from_slice(&byte).unwrap();


    let data = result.data
        .get("slug").unwrap();

    let data = match data {
        None => { Value::Null }
        Some(data) => { data.to_owned() }
    };

    assert_eq!(data, slug);
}