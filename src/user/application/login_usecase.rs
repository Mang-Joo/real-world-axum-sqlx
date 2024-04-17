use std::sync::Arc;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::app_state::AppState;
use crate::auth::jwt_encoder::JwtEncoder;
use crate::user::application::repository::find_by_email;
use crate::user::domain::hash_password::ArgonHash;
use crate::user::domain::user::User;

pub async fn user_login(app_state: Arc<AppState>, login_request: LoginRequest) -> anyhow::Result<UserResponse> {
    let user = find_by_email(&login_request.email.unwrap(), &app_state.pool)
        .await
        .map_err(|err| anyhow!(err))?;

    if user.not_verify_password(login_request.password.unwrap(), &ArgonHash::default()).await {
        return Err(anyhow!("Not equal password."));
    }

    let jwt_encoder = JwtEncoder::from(app_state);
    let token = jwt_encoder.encode_jwt(user.email())
        .await?;

    Ok(to_response(user, token).await)
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(required(message = "Email is required."), email(message = "Must be form email."))]
    pub email: Option<String>,
    #[validate(required(message = "Password is required."), length(min = 6, message = "Password must be at least 6 characters."))]
    pub password: Option<String>,
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