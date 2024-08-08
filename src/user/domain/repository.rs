use std::sync::Arc;

use axum::async_trait;

use crate::config::RealWorldResult;

use super::{
    model::{UserRegistry, UserUpdate},
    user::User,
};

pub type DynUserRepository = Arc<dyn UserRepository + Send + Sync>;

#[async_trait]
pub trait UserRepository {
    async fn registry(&self, user_register: UserRegistry) -> RealWorldResult<User>;
    async fn is_exist(&self, user_email: String) -> RealWorldResult<bool>;
    async fn find_by_email(&self, email: String) -> RealWorldResult<User>;
    async fn find_by_id(&self, id: i64) -> RealWorldResult<User>;
    async fn update(&self, id: i64, user_update: UserUpdate) -> RealWorldResult<User>;
}
