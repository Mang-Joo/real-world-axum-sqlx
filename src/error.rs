use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[repr(i32)]
#[derive(Error, Debug)]
pub enum AppError {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized(String) = 40001,
    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden = 40003,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound = 40004,

    #[error(transparent)]
    AnyHow(#[from] anyhow::Error) = 40000,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized(_) => { StatusCode::UNAUTHORIZED }
            AppError::Forbidden => { StatusCode::FORBIDDEN }
            AppError::NotFound => { StatusCode::NOT_FOUND }
            AppError::AnyHow(_) => { StatusCode::BAD_REQUEST }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), Json(ErrorResponse::new(&self))).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: i32,
    message: String,
}

impl ErrorResponse {
    fn new(app_error: &AppError) -> Self {
        ErrorResponse {
            code: app_error.status_code().as_u16() as i32,
            message: app_error.to_string(),
        }
    }
}

pub async fn error_handler(err: AppError) -> impl IntoResponse {
    err.into_response()
}