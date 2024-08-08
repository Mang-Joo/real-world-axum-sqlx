use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::config::error::AppError;
use crate::config::validate::ValidationExtractor;

use super::domain::service::DynUserService;
use super::domain::user::AuthUser;
use super::service::model::UserRegistry;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRequestDto<T: Validate> {
    #[validate(nested)]
    user: T,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponseDto<T> {
    user: T,
}

pub async fn register_api(
    Extension(service): Extension<DynUserService>,
    ValidationExtractor(request): ValidationExtractor<UserRequestDto<UserRegisterApiRequest>>,
) -> Result<Json<UserResponseDto<UserResponse>>, AppError> {
    let user_registry = request.user;

    let _ = service
        .is_exist(user_registry.email.clone().unwrap())
        .await
        .map_err(|err| {
            return AppError::AnyHow(err);
        })?;

    let auth_user = service
        .registry(user_registry.to_registry())
        .await
        .map_err(|err| {
            return AppError::AnyHow(err);
        })?;

    let response = UserResponse::new(auth_user);
    Ok(Json(UserResponseDto { user: response }))
}

#[derive(Deserialize, Validate)]
pub struct UserRegisterApiRequest {
    #[validate(length(min = 1, message = "User Name is required."))]
    username: Option<String>,
    #[validate(length(min = 1, message = "Email is required."))]
    email: Option<String>,
    #[validate(length(min = 8, message = "Password is required."))]
    password: Option<String>,
}
impl UserRegisterApiRequest {
    fn to_registry(self) -> UserRegistry {
        UserRegistry::new(
            self.username.unwrap(),
            self.email.unwrap(),
            self.password.unwrap(),
        )
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl UserResponse {
    pub fn new(auth_user: AuthUser) -> Self {
        Self {
            email: auth_user.email().to_owned(),
            token: auth_user.token().to_owned(),
            username: auth_user.username().to_owned(),
            bio: auth_user.bio(),
            image: auth_user.image(),
        }
    }
}
