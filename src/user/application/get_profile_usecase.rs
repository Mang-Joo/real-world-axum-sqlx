use std::sync::Arc;
use crate::config;
use crate::config::app_state::{AppState, ArcAppState};

use crate::user::application::follow_repository::is_follow;
use crate::user::application::user_repository::find_by_user_name;
use crate::user::domain::user::User;

pub async fn get_profile(
    app_state: ArcAppState,
    user_id: Option<i64>,
    user_name: String,
) -> config::Result<(User, bool)> {
    let target_user = find_by_user_name(&user_name, Arc::clone(&app_state)).await?;

    let is_follow: bool = if let Some(user_id) = user_id {
        is_follow(user_id, target_user.id(), Arc::clone(&app_state)).await
    } else { false };

    Ok((target_user, is_follow))
}

