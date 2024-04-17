use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::extract::State;
use log::info;
use serde::Deserialize;
use validator_derive::Validate;

use crate::app_state::AppState;
use crate::auth::jwt_encoder;
use crate::auth::jwt_encoder::JwtEncoder;
use crate::user::application::login_usecase::{to_response, UserResponse};
use crate::user::application::repository::{find_by_email, save_user};
use crate::user::domain::user::User;
use crate::validate::ValidationExtractor;

pub async fn registration(
    state: Arc<AppState>,
    request: RegistrationUserRequest,
) -> anyhow::Result<UserResponse> {
    let user = request.to_domain();
    info!("registration request email is {}", user.email());

    let find_user = find_by_email(user.email(), &state.pool)
        .await;

    match find_user {
        Ok(_) => { return Err(anyhow!("Already has email.")) }
        Err(_) => { }
    };



    let user = save_user(user, &state.pool)
        .await?;

    let jwt_encoder = JwtEncoder::from(state);
    let token = jwt_encoder.encode_jwt(user.email()).await?;

    let response = to_response(user, token)
        .await;

    info!("Success Registration! {}", &response.email);

    Ok(response)
}


#[derive(Deserialize, Validate, Debug)]
pub struct RegistrationUserRequest {
    #[validate(required(message = "name is required."))]
    username: Option<String>,
    #[validate(required(message = "email is required."))]
    email: Option<String>,
    #[validate(required(message = "password is required."), length(min = 6, message = "Password must be at least 6 characters."))]
    password: Option<String>,
    bio: Option<String>,
    image: Option<String>,

}

impl RegistrationUserRequest {
    fn to_domain(self) -> User {
        User::new(
            0,
            self.email.unwrap(),
            self.password.unwrap(),
            self.username.unwrap(),
            self.bio,
            self.image,
        )
    }
}