use std::sync::Arc;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app_state::AppState;
use crate::auth::jwt_encoder::JwtEncoder;
use crate::user::application::repository::find_by_email;
use crate::user::domain::hash_password::ArgonHash;
use crate::user::domain::user::User;

pub async fn user_login(app_state: Arc<AppState>, login_request: LoginRequest) -> anyhow::Result<LoginResponse> {
    let user = find_by_email(&login_request.email, &app_state.pool)
        .await
        .map_err(|err| anyhow!(err))?;

    if user.not_verify_password(login_request.password, &ArgonHash::default()).await {
        return Err(anyhow!("Not equal password."));
    }

    let jwt_encoder = JwtEncoder::from(app_state);
    let token = jwt_encoder.encode_jwt(&user.email)
        .await?;

    Ok(to_response(user, token).await)
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    email: String,
    token: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

async fn to_response(user: User, token: String) -> LoginResponse {
    LoginResponse {
        email: user.email,
        token,
        username: user.user_name,
        bio: user.bio,
        image: user.image,
    }
}