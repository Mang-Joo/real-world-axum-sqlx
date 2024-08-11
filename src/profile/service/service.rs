use anyhow::anyhow;
use axum::async_trait;
use log::{error, info};

use crate::{
    config::RealWorldResult,
    profile::{
        self,
        domain::{profile::Profile, repository::DynProfileRepository, service::ProfileService},
    },
    user::domain::service::DynUserService,
};

pub struct ConcreteProfileServce {
    repository: DynProfileRepository,
    user_service: DynUserService,
}

impl ConcreteProfileServce {
    pub fn new(repository: DynProfileRepository, user_service: DynUserService) -> Self {
        Self {
            repository,
            user_service,
        }
    }
}

#[async_trait]
impl ProfileService for ConcreteProfileServce {
    async fn get_profile(
        &self,
        optional_user: Option<i64>,
        username: String,
    ) -> RealWorldResult<Profile> {
        info!("[Get Profile] username is {}", &username);
        let user = self.user_service.get_info_by_user_name(username).await?;

        match optional_user {
            Some(user_id) => {
                let is_follow = self.repository.is_follow(user_id, user.id()).await?;
                let profile = Profile::new(
                    user.user_name().to_owned(),
                    user.bio().to_owned(),
                    user.image().to_owned(),
                    is_follow,
                );
                Ok(profile)
            }
            None => {
                let profile = Profile::new(
                    user.user_name().to_owned(),
                    user.bio().to_owned(),
                    user.image().to_owned(),
                    false,
                );
                Ok(profile)
            }
        }
    }
}
