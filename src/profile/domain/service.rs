use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

use super::profile::Profile;

pub type DynProfileService = Arc<dyn ProfileService + Send + Sync>;

#[async_trait]
pub trait ProfileService {
    async fn get_profile(
        &self,
        optional_user: Option<i64>,
        username: String,
    ) -> RealWorldResult<Profile>;
}
