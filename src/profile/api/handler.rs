use axum::{extract::Path, Extension, Json};

use crate::{
    config::{error::AppError, validate::OptionalAuthenticateExtractor},
    profile::domain::service::DynProfileService,
};

use super::model::{ProfileResponse, ProfileResponseDto};

pub async fn get_profile(
    OptionalAuthenticateExtractor(id): OptionalAuthenticateExtractor,
    Extension(service): Extension<DynProfileService>,
    Path(username): Path<String>,
) -> Result<Json<ProfileResponseDto<ProfileResponse>>, AppError> {
    let profile = service.get_profile(id, username).await?;

    let response = ProfileResponse::new(profile);

    let response = ProfileResponseDto { profile: response };

    Ok(Json(response))
}
