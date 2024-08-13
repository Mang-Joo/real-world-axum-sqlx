use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

use super::model::Profile;

pub type DynProfileService = Arc<dyn ProfileService + Send + Sync>;

#[async_trait]
pub trait ProfileService {
    async fn get_profile(
        &self,
        optional_user: Option<i64>,
        username: String,
    ) -> RealWorldResult<Profile>;

    async fn follow_user(
        &self,
        follower_id: i64,
        following_username: String,
    ) -> RealWorldResult<Profile>;

    async fn unfollow(
        &self,
        follower_id: i64,
        following_username: String,
    ) -> RealWorldResult<Profile>;
}
