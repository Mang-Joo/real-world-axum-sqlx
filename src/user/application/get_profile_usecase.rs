use std::sync::Arc;

use crate::app_state;
use crate::app_state::AppState;
use crate::user::application::follow_repository::is_follow;
use crate::user::application::user_repository::find_by_user_name;
use crate::user::domain::user::User;

pub async fn get_profile(
    app_state: Arc<AppState>,
    user_id: Option<i64>,
    user_name: String,
) -> app_state::Result<(User, bool)> {
    let target_user = find_by_user_name(&user_name, &app_state.pool).await?;

    let is_follow: bool = if let Some(user_id) = user_id {
        is_follow(user_id, target_user.id(), &app_state.pool).await
    } else { false };

    Ok((target_user, is_follow))
}

