use std::sync::Arc;

use anyhow::anyhow;

use crate::app_state;
use crate::app_state::AppState;
use crate::user::application::follow_repository::{is_follow, save_follow};
use crate::user::application::user_repository::find_by_user_name;
use crate::user::domain::user::User;

pub async fn follow_user(
    follow_user_id: i64,
    follower_user_name: String,
    app_state: Arc<AppState>,
) -> app_state::Result<(User, bool)> {
    let pool = &app_state.pool;

    let target_user = find_by_user_name(&follower_user_name, pool)
        .await?;

    let is_follow = is_follow(follow_user_id, target_user.id(), pool)
        .await;

    if is_follow {
        return Err(anyhow!("Already follow {}", target_user.user_name()));
    };

    let follow = save_follow(follow_user_id, target_user.id(), pool).await?;

    Ok((target_user, follow))
}