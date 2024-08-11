use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

use super::{
    model::{UserLogin, UserRegistry, UserUpdate},
    user::{AuthUser, User},
};

pub type DynUserService = Arc<dyn UserService + Send + Sync>;

#[async_trait]
pub trait UserService {
    async fn registry(&self, user_registry: UserRegistry) -> RealWorldResult<AuthUser>;
    async fn is_exist(&self, email: String) -> RealWorldResult<()>;
    async fn login(&self, login: UserLogin) -> RealWorldResult<AuthUser>;
    async fn get_info(&self, id: i64) -> RealWorldResult<AuthUser>;
    async fn update(&self, id: i64, user_update: UserUpdate) -> RealWorldResult<AuthUser>;
    async fn get_info_by_user_name(&self, username: String) -> RealWorldResult<User>;
}
