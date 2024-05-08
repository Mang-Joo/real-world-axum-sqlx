use anyhow::anyhow;
use chrono::Utc;
use log::{error, info};
use sqlx::Row;

use crate::config;
use crate::config::app_state::ArcAppState;
use crate::config::db::DbPool;

pub async fn is_follow(
    follower_id: i64,
    following_id: i64,
    arc_app_state: ArcAppState) -> bool {
    let result: bool = sqlx::query("
            SELECT EXISTS(
                SELECT 1
                FROM user_follow
                WHERE follower_id = $1
                AND following_id = $2
            )
    ")
        .bind(follower_id)
        .bind(following_id)
        .fetch_one(&arc_app_state.pool)
        .await
        .map_err(|err| {
            error!("follow checking error {}", err.to_string());
            false
        })
        .unwrap()
        .get(0);

    info!("user's id {} following is {}", follower_id, result);

    result
}

pub async fn save_follow(
    follower_id: i64,
    following_id: i64,
    arc_app_state: ArcAppState
) -> config::Result<bool> {
    let _ = sqlx::query("
    INSERT INTO user_follow (follower_id, following_id, created_at, updated_at, deleted)
    VALUES ($1, $2, $3 , $4, $5)
    ")
        .bind(follower_id)
        .bind(following_id)
        .bind(Utc::now())
        .bind(Utc::now())
        .bind(false)
        .execute(&arc_app_state.pool)
        .await
        .map_err(|err| {
            error!("Following error follower id is {} and following id is {} \n Error is {}", follower_id, following_id, err);
            anyhow!("Following User Error {}", err)
        })?;

    Ok(true)
}

pub async fn unfollow(
    follower_id: i64,
    following_id: i64,
    arc_app_state: ArcAppState
) -> config::Result<bool> {
    let _ = sqlx::query(r#"
    UPDATE user_follow
    SET updated_at = ?, deleted = true
    WHERE follower_id = ? AND following_Id = ?
    "#)
        .bind(Utc::now())
        .bind(follower_id)
        .bind(following_id)
        .execute(&arc_app_state.pool)
        .await
        .map_err(|err| {
            error!("Unfollow Error {}", err);
            anyhow!("Unfollow error {}", err)
        })?;

    Ok(false)
}


mod test {
    use std::sync::Arc;
    use crate::config::app_state;
    use crate::config::app_state::init_app_state;
    use crate::config::db::init_db;
    use crate::user::application::follow_repository::{is_follow, save_follow, unfollow};
    use crate::user::application::user_repository::find_by_id;

    #[tokio::test]
    async fn is_follow_user_false_test() {
        let app_state = init_app_state().await;
        let arc_app_state = Arc::new(app_state);


        let user = find_by_id(1, arc_app_state)
            .await
            .expect("");

        let response = is_follow(user.id(), 2, arc_app_state).await;

        assert_eq!(false, response);
    }

    #[tokio::test]
    async fn save_following_test() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres"))
            .await;

        let result = save_follow(1, 2, &db).await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn unfollow_success_test() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres"))
            .await;

        let result = unfollow(1, 2, &db).await;

        assert_eq!(result.is_ok(), true);
    }
}