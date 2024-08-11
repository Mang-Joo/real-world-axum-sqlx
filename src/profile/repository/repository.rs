use axum::async_trait;

use crate::{
    config::{db::DbPool, RealWorldResult},
    profile::domain::repository::ProfileRepository,
};

pub struct ConcreteProfileRepository {
    pool: DbPool,
}

impl ConcreteProfileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
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
            ",
            follower_id,
            following_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(is_follow.is_some())
    }
}
