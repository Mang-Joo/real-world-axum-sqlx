use axum::{extract::Path, Extension, Json};

use crate::{
    config::{
        error::AppError,
        validate::{JwtValidationExtractor, OptionalAuthenticateExtractor},
    },
    profile::{self, domain::service::DynProfileService},
};

use super::model::{ProfileResponse, ProfileResponseDto};

pub async fn get_profile(
    OptionalAuthenticateExtractor(id): OptionalAuthenticateExtractor,
    Extension(service): Extension<DynProfileService>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponseDto<ProfileResponse>>, AppError> {
    let profile = service.get_profile(id, username).await?;

    let response = ProfileResponse::from(profile);

    let response = ProfileResponseDto { profile: response };

    Ok(Json(response))
}

pub async fn follow_user_api(
    JwtValidationExtractor(follower_id): JwtValidationExtractor,
    Extension(service): Extension<DynProfileService>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponseDto<ProfileResponse>>, AppError> {
    let profile = service.follow_user(follower_id, username).await?;

    let response = ProfileResponse::from(profile);

    let response = ProfileResponseDto { profile: response };

    Ok(Json(response))
}

pub async fn unfollow_api(
    JwtValidationExtractor(follower_id): JwtValidationExtractor,
    Extension(service): Extension<DynProfileService>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponseDto<ProfileResponse>>, AppError> {
    let profile = service.ungfollow(follower_id, username).await?;

    let response = ProfileResponse::from(profile);

    let response = ProfileResponseDto { profile: response };

    Ok(Json(response))
}
