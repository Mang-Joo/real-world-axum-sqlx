use std::sync::Arc;

use serde::Deserialize;
use validator_derive::Validate;

use crate::config;
use crate::config::app_state::AppState;
use crate::user::application::user_repository;
use crate::user::application::user_repository::update_user_entity;
use crate::user::domain::hash_password::ArgonHash;
use crate::user::domain::user::User;

pub async fn update_user(
    user_id: i64,
    app_state: &Arc<AppState>,
    request: UpdateUserRequest,
) -> config::Result<User> {
    let user = user_repository::find_by_id(user_id, &app_state.pool).await?;

    let updated_user = user.set_user_name(request.user_name.unwrap())
        .set_email(request.email.unwrap())
        .set_password(request.password.unwrap())
        .set_image(request.image)
        .set_bio(request.bio);

    let updated_user = updated_user.hash_password(&ArgonHash::default())
        .await?;

    let _ = update_user_entity(&updated_user, &app_state.pool).await?;

    Ok(updated_user)
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUserRequest {
    #[validate(required(message = "name is required."))]
    user_name: Option<String>,
    #[validate(required(message = "email is required."))]
    email: Option<String>,
    #[validate(required(message = "password is required."))]
    password: Option<String>,
    image: Option<String>,
    bio: Option<String>,
}