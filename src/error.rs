use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = AppError> = anyhow::Result<T, E>;


pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Self::Body> {
        (
            self.status,
            format!("{{\"error\":\"{}\"}}", self.message).into(),
        )
            .into_response()
    }
}