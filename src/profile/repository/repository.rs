use anyhow::anyhow;
use axum::async_trait;

use crate::{
    config::{db::DbPool, RealWorldResult},
    profile::domain::repository::ProfileRepository,
};

pub struct ConcreteProfileRepository {
    db_pool: DbPool,
}

impl ConcreteProfileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { db_pool: pool }
    }
}

#[async_trait]
impl ProfileRepository for ConcreteProfileRepository {
    async fn is_follow(&self, follower_id: i64, following_id: i64) -> RealWorldResult<bool> {
        let is_follow = sqlx::query!(
            "
            SELECT 1 as is_follow
            FROM user_follow
            WHERE follower_id = $1
            AND following_id = $2
            AND DELETED = false
            ",
            follower_id,
            following_id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(is_follow.is_some())
    }

    async fn follow_user(&self, follower_id: i64, following_id: i64) -> RealWorldResult<()> {
        let result = sqlx::query!(
            "
            INSERT INTO user_follow (follower_id, following_id)
            VALUES ($1, $2)
            ",
            follower_id,
            following_id
        )
        .execute(&self.db_pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow!("{}", err)),
        }
    }

    async fn unfollow(&self, follower_id: i64, following_id: i64) -> RealWorldResult<()> {
        let result = sqlx::query!(
            "
            UPDATE user_follow SET deleted = true
            WHERE follower_id = $1
            AND following_id = $2
            ",
            follower_id,
            following_id
        )
        .execute(&self.db_pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow!("{}", err)),
        }
    }
}
