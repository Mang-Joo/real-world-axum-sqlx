use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use log::info;
use serde::Deserialize;
use validator_derive::Validate;

use crate::app_state::AppState;
use crate::app_state::Result;
use crate::user::application::user_repository::{find_by_email, find_by_user_name, save_user};
use crate::user::domain::hash_password::ArgonHash;
use crate::user::domain::user::User;

pub async fn registration(
    state: &Arc<AppState>,
    request: RegistrationUserRequest,
) -> Result<User> {
    let user = request.to_domain();
    info!("registration request email is {}", user.email());

    let find_user = find_by_email(user.email(), &state.pool)
        .await;

    match find_user {
        Ok(_) => { return Err(anyhow!("Already has email.")); }
        Err(_) => {}
    };

    let find_user = find_by_user_name(user.user_name(), &state.pool).await;

    match find_user {
        Ok(_) => { return Err(anyhow!("Already has username.")); }
        Err(_) => {}
    };

    let user = user.hash_password(&ArgonHash::default()).await?;
    let user = save_user(
        user,
        &state.pool,
    )
        .await?;

    info!("Success Registration! {}", user.email());

    Ok(user)
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
            Utc::now(),
            Utc::now(),
        )
    }
}