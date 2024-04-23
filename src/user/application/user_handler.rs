use std::sync::Arc;

use axum::{Extension, Json};
use axum::response::IntoResponse;
use serde::Serialize;

use crate::app_state::AppState;
use crate::auth::jwt_encoder::JwtEncoder;
use crate::error::AppError;
use crate::user::application::{get_current_user_usecase, login_usecase, registration_usecase};
use crate::user::application::login_usecase::LoginRequest;
use crate::user::application::registration_usecase::RegistrationUserRequest;
use crate::user::application::update_user_usecase::{update_user, UpdateUserRequest};
use crate::user::domain::user::User;
use crate::validate::{JwtValidationExtractor, ValidationExtractor};

pub async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    ValidationExtractor(payload): ValidationExtractor<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = login_usecase::user_login(state, payload)
        .await
        .map_err(|err| {
            return AppError::AnyHow(err);
        })?;

    Ok(Json(response))
}

pub async fn registration_user(
    Extension(state): Extension<Arc<AppState>>,
    ValidationExtractor(request): ValidationExtractor<RegistrationUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = registration_usecase::registration(
        &state,
        request,
    ).await?;

    let encoder = JwtEncoder::from(state);
    let token = encoder.encode_jwt(&user).await?;

    let response = to_response(user, token).await;

    Ok(Json(response))
}

pub async fn get_current_user(
    Extension(state): Extension<Arc<AppState>>,
    JwtValidationExtractor(user_id): JwtValidationExtractor,
) -> Result<impl IntoResponse, AppError> {
    let user = get_current_user_usecase::get_current_user(user_id, state).await?;

    let response = to_response_by_user(user).await;

    Ok(Json(response))
}

pub async fn update_user_handler(
    JwtValidationExtractor(user_id): JwtValidationExtractor,
    Extension(state): Extension<Arc<AppState>>,
    ValidationExtractor(request): ValidationExtractor<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let updated_user = update_user(user_id, &state, request).await?;

    let response = to_response_by_user(updated_user).await;

    Ok(Json(response))
}

#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

pub async fn to_response(user: User, token: String) -> UserResponse {
    UserResponse {
        email: user.email().to_owned(),
        token,
        username: user.user_name().to_owned(),
        bio: user.bio().to_owned(),
        image: user.image().to_owned(),
    }
}

pub async fn to_response_by_user(user: User) -> UserResponse {
    UserResponse {
        email: user.email().to_owned(),
        token: String::new(),
        username: user.user_name().to_owned(),
        bio: user.bio().to_owned(),
        image: user.image().to_owned(),
    }
}