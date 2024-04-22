use std::sync::Arc;

use log::info;

use crate::app_state;
use crate::app_state::AppState;
use crate::user::application::repository;
use crate::user::domain::user::User;

pub async fn get_current_user(user_id: i64, app_state: Arc<AppState>) -> app_state::Result<User> {
    let user = repository::find_by_id(user_id, &app_state.pool).await?;
    info!("Success get user");

    Ok(user)
}