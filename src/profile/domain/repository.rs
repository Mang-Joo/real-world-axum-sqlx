use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

pub type DynProfileRepository = Arc<dyn ProfileRepository + Send + Sync>;

#[async_trait]
pub trait ProfileRepository {
    async fn is_follow(&self, follower_id: i64, following_id: i64) -> RealWorldResult<bool>;
    async fn follow_user(&self, follower_id: i64, following_id: i64) -> RealWorldResult<()>;
    async fn unfollow(&self, follower_id: i64, following_id: i64) -> RealWorldResult<()>;
}
