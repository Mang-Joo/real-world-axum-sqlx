use std::sync::Arc;

use log::info;

use crate::config;
use crate::config::app_state::AppState;
use crate::user::application::user_repository;
use crate::user::domain::user::User;

pub async fn get_current_user(user_id: i64, app_state: Arc<AppState>) -> config::Result<User> {
    let user = user_repository::find_by_id(user_id, &app_state.pool).await?;
    info!("Success get user");

    Ok(user)
}