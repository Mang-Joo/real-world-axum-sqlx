use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::config::error::AppError;
use crate::config::validate::{self, JwtValidationExtractor, ValidationExtractor};

use super::domain::model::{UserLogin, UserRegistry, UserUpdate};
use super::domain::service::DynUserService;
use super::domain::user::AuthUser;

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

    let _ = service.is_exist(user_registry.email.clone()).await?;

    let auth_user = service
        .registry(user_registry.to_registry())
        .await
        .map_err(|err| {
            return AppError::AnyHow(err);
        })?;

    let response = UserResponse::new(auth_user);
    Ok(Json(UserResponseDto { user: response }))
}

pub async fn login_api(
    Extension(service): Extension<DynUserService>,
    ValidationExtractor(request): ValidationExtractor<UserRequestDto<UserLoginRequest>>,
) -> Result<Json<UserResponseDto<UserResponse>>, AppError> {
    let auth_user = service.login(request.user.to_login()).await?;

    let response = UserResponse::new(auth_user);
    Ok(Json(UserResponseDto { user: response }))
}

pub async fn get_info_api(
    JwtValidationExtractor(id): JwtValidationExtractor,
    Extension(service): Extension<DynUserService>,
) -> Result<Json<UserResponseDto<UserResponse>>, AppError> {
    let auth_user = service.get_info(id).await?;

    let response = UserResponse::new(auth_user);
    Ok(Json(UserResponseDto { user: response }))
}

pub async fn update_user_api(
    JwtValidationExtractor(id): JwtValidationExtractor,
    Extension(service): Extension<DynUserService>,
    ValidationExtractor(request): ValidationExtractor<UserRequestDto<UserUpdateApiRequest>>,
) -> Result<Json<UserResponseDto<UserResponse>>, AppError> {
    let user_update_api_request = request.user;
    if user_update_api_request.email.clone().is_some() {
        service
            .is_exist(user_update_api_request.email.clone().unwrap())
            .await?;
    }

    let auth_user = service
        .update(id, user_update_api_request.to_update())
        .await?;

    let response = UserResponse::new(auth_user);
    Ok(Json(UserResponseDto { user: response }))
}

#[derive(Deserialize, Validate)]
pub struct UserRegisterApiRequest {
    #[validate(length(min = 1, message = "User Name is required."))]
    username: String,
    #[validate(length(min = 1, message = "Email is required."))]
    email: String,
    #[validate(length(min = 8, message = "Password is wrong."))]
    password: String,
}
impl UserRegisterApiRequest {
    fn to_registry(self) -> UserRegistry {
        UserRegistry::new(self.username, self.email, self.password)
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

#[derive(Validate, Deserialize)]
pub struct UserLoginRequest {
    #[validate(length(min = 1, message = "Email is required."))]
    email: String,
    #[validate(length(min = 8, message = "Password is wrong."))]
    password: String,
}

impl UserLoginRequest {
    fn to_login(self) -> UserLogin {
        UserLogin::new(self.email, self.password)
    }
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateApiRequest {
    #[validate(length(min = 1, message = "User Name is required."))]
    username: Option<String>,
    #[validate(length(min = 1, message = "Email is required."))]
    email: Option<String>,
    #[validate(length(min = 8, message = "Password is wrong."))]
    password: Option<String>,
    image: Option<String>,
    bio: Option<String>,
}
impl UserUpdateApiRequest {
    fn to_update(self) -> UserUpdate {
        UserUpdate::new(
            self.email,
            self.username,
            self.password,
            self.image,
            self.bio,
        )
    }
}
