use anyhow::Ok;
use axum::async_trait;
use log::{error, info};

use crate::{
    config::RealWorldResult,
    profile::domain::{model::Profile, repository::DynProfileRepository, service::ProfileService},
    user::domain::service::DynUserService,
};

pub struct ConcreteProfileService {
    repository: DynProfileRepository,
    user_service: DynUserService,
}

impl ConcreteProfileService {
    pub fn new(repository: DynProfileRepository, user_service: DynUserService) -> Self {
        Self {
            repository,
            user_service,
        }
    }
}

#[async_trait]
impl ProfileService for ConcreteProfileService {
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

    async fn follow_user(
        &self,
        follower_id: i64,
        following_username: String,
    ) -> RealWorldResult<Profile> {
        let following_user = self
            .user_service
            .get_info_by_user_name(following_username)
            .await?;

        let result = self
            .repository
            .is_follow(follower_id, following_user.id())
            .await;

        match result {
            RealWorldResult::Ok(is_follow) => {
                if is_follow {
                    info!("Already follow {} ", following_user.user_name());
                    let profile = Profile::new(
                        following_user.user_name().to_owned(),
                        following_user.bio().to_owned(),
                        following_user.image().to_owned(),
                        true,
                    );
                    RealWorldResult::Ok(profile)
                } else {
                    info!("Request Follow");
                    let follow_user = self
                        .repository
                        .follow_user(follower_id, following_user.id())
                        .await;
                    match follow_user {
                        RealWorldResult::Ok(_) => {
                            info!("Follow Success {}", following_user.user_name());
                            let profile = Profile::new(
                                following_user.user_name().to_owned(),
                                following_user.bio().to_owned(),
                                following_user.image().to_owned(),
                                true,
                            );
                            RealWorldResult::Ok(profile)
                        }
                        Err(err) => {
                            error!("Follow failed Error is {}", err);
                            Err(err)
                        }
                    }
                }
            }
            Err(err) => {
                error!("Could not find is_follow follower id is {}", follower_id);
                Err(err)
            }
        }
    }

    async fn unfollow(
        &self,
        follower_id: i64,
        following_username: String,
    ) -> RealWorldResult<Profile> {
        let follower_user = self
            .user_service
            .get_info_by_user_name(following_username)
            .await?;

        info!("Find follower user {} ", follower_user.user_name());

        let _unfollow = self
            .repository
            .unfollow(follower_id, follower_user.id())
            .await;

        Ok(Profile::new(
            follower_user.user_name().to_string(),
            follower_user.bio().to_owned(),
            follower_user.image().to_owned(),
            false,
        ))
    }
}
