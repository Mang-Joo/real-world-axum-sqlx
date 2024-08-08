use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

use super::{
    model::{UserLogin, UserRegistry},
    user::AuthUser,
};

pub type DynUserService = Arc<dyn UserService + Send + Sync>;

#[async_trait]
pub trait UserService {
    async fn registry(&self, user_registry: UserRegistry) -> RealWorldResult<AuthUser>;
    async fn is_exist(&self, email: String) -> RealWorldResult<()>;
    async fn login(&self, login: UserLogin) -> RealWorldResult<AuthUser>;
}
