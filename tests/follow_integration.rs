use std::collections::HashMap;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use serde::Deserialize;
use tower::ServiceExt;
use bytes::Bytes;

use crate::common::{fixture_route, TOKEN_FIXTURE};

mod common;

#[tokio::test]
pub async fn unfollow_api_is_400_bad_request() {
    let request = Request::builder()
        .uri("/api/profiles/mangjoo22/follow")
        .method(Method::DELETE)
        .header("Content-Type", "application/json")
        .header("Authorization", common::TOKEN_FIXTURE)
        .body(Body::empty())
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();


    let request = Request::builder()
        .uri("/api/profiles/mangjoo22/follow")
        .method(Method::DELETE)
        .header("Content-Type", "application/json")
        .header("Authorization", common::TOKEN_FIXTURE)
        .body(Body::empty())
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST)
}

#[tokio::test]
pub async fn unfollow_api_is_200_ok() {
    let request = Request::builder()
        .uri("/api/profiles/mangjoo22/follow")
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", TOKEN_FIXTURE)
        .body(Body::empty())
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    let request = Request::builder()
        .uri("/api/profiles/mangjoo22/follow")
        .method(Method::DELETE)
        .header("Content-Type", "application/json")
        .header("Authorization", TOKEN_FIXTURE)
        .body(Body::empty())
        .unwrap();

    let response = fixture_route()
        .await
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    #[serde(flatten)]
    data: HashMap<String, Option<String>>,
}