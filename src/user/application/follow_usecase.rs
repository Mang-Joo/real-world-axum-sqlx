use std::sync::Arc;

use anyhow::anyhow;

use crate::config;
use crate::config::app_state::AppState;
use crate::user::application::follow_repository::{is_follow, save_follow, unfollow};
use crate::user::application::user_repository::find_by_user_name;
use crate::user::domain::user::User;

pub async fn follow_user(
    follow_user_id: i64,
    following_user_name: String,
    app_state: Arc<AppState>,
) -> config::Result<(User, bool)> {
    let pool = &app_state.pool;

    let target_user = find_by_user_name(&following_user_name, pool)
        .await?;

    let is_follow = is_follow(follow_user_id, target_user.id(), pool)
        .await;

    if is_follow {
        return Err(anyhow!("Already follow {}", target_user.user_name()));
    };

    let follow = save_follow(follow_user_id, target_user.id(), pool).await?;

    Ok((target_user, follow))
}

pub async fn unfollow_user(
    follow_user_id: i64,
    following_user_name: String,
    app_state: Arc<AppState>,
) -> config::Result<(User, bool)> {
    let pool = &app_state.pool;

    let target_user = find_by_user_name(&following_user_name, pool)
        .await?;

    let is_follow = is_follow(follow_user_id, target_user.id(), pool)
        .await;

    if is_follow {
        let is_follow = unfollow(follow_user_id, target_user.id(), pool)
            .await?;

        Ok((target_user, is_follow))
    } else {
        return Err(anyhow!("Already unfollow user"));
    }
}
