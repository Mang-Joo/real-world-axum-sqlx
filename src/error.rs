use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

const BAD_REQUEST: u16 = 40000;
const UNAUTHORIZED_ERROR_CODE: u16 = 40001;
const VALIDATE_ERROR_CODE: u16 = 40002;
const FORBIDDEN_ERROR_CODE: u16 = 40003;

#[derive(Error, Debug)]
pub enum AppError {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,
    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    #[error(transparent)]
    AnyHow(#[from] anyhow::Error),

    #[error(transparent)]
    ValidateError(#[from] ValidationErrors),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => { StatusCode::UNAUTHORIZED }
            AppError::Forbidden => { StatusCode::FORBIDDEN }
            AppError::AnyHow(_) => { StatusCode::BAD_REQUEST }
            AppError::ValidateError(_) => { StatusCode::BAD_REQUEST }
        }
    }

    fn error_code(&self) -> u16 {
        match self {
            AppError::Unauthorized => { UNAUTHORIZED_ERROR_CODE }
            AppError::Forbidden => { FORBIDDEN_ERROR_CODE }
            AppError::AnyHow(_) => { BAD_REQUEST }
            AppError::ValidateError(_) => { VALIDATE_ERROR_CODE }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), Json(ErrorResponse::new(&self)))
            .into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl ErrorResponse {
    fn new(app_error: &AppError) -> Self {
        ErrorResponse {
            code: app_error.error_code(),
            message: app_error.to_string(),
        }
    }
}

pub async fn error_handler(err: AppError) -> impl IntoResponse {
    err.into_response()
}