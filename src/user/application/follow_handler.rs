use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::config::app_state::AppState;
use crate::config::error::AppError;
use crate::config::validate::{JwtValidationExtractor, OptionalAuthenticateExtractor};
use crate::user::application::{follow_usecase, get_profile_usecase};
use crate::user::domain::user::User;

pub async fn get_profile_handler(
    OptionalAuthenticateExtractor(user_id): OptionalAuthenticateExtractor,
    Extension(state): Extension<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (user, is_follow) = get_profile_usecase::get_profile(state, user_id, user_name).await?;

    let response = ProfileResponse::new(user, is_follow);

    Ok(Json(response))
}

pub async fn follow_user_handler(
    JwtValidationExtractor(user_id): JwtValidationExtractor,
    Extension(state): Extension<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (user, is_follow) = follow_usecase::follow_user(user_id, user_name, state).await?;

    let response = ProfileResponse::new(user, is_follow);

    Ok(Json(response))
}

pub async fn unfollow_user_handler(
    JwtValidationExtractor(user_id): JwtValidationExtractor,
    Extension(state): Extension<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let (user, is_follow) = follow_usecase::unfollow_user(user_id, user_name, state)
        .await?;

    let response = ProfileResponse::new(user, is_follow);

    Ok(Json(response))
}

#[derive(Serialize)]
pub struct ProfileResponse {
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl ProfileResponse {
    fn new(
        user: User,
        is_follow: bool,
    ) -> Self {
        ProfileResponse {
            user_name: user.user_name().to_owned(),
            bio: user.bio().to_owned(),
            image: user.image().to_owned(),
            following: is_follow,
        }
    }
}