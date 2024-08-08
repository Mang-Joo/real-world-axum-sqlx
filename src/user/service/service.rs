use std::borrow::Borrow;

use anyhow::anyhow;
use axum::async_trait;
use log::{error, info};
use validator::ValidateRequired;

use crate::{
    auth::{hash_password::DynHashPassword, jwt_encoder::ArcJwtEncoder},
    config::RealWorldResult,
    user::domain::{
        model::{UserLogin, UserRegistry, UserUpdate},
        repository::DynUserRepository,
        service::UserService,
        user::AuthUser,
    },
};

pub struct ConcreteUserService {
    repository: DynUserRepository,
    hash_password: DynHashPassword,
    jwt_encoder: ArcJwtEncoder,
}

impl ConcreteUserService {
    pub fn new(
        repository: DynUserRepository,
        hash_password: DynHashPassword,
        jwt_encoder: ArcJwtEncoder,
    ) -> Self {
        Self {
            repository,
            hash_password,
            jwt_encoder,
        }
    }
}

#[async_trait]
impl UserService for ConcreteUserService {
    async fn registry(&self, user_registry: UserRegistry) -> RealWorldResult<AuthUser> {
        let hashed_user_registry = user_registry.hash_of_password(&self.hash_password)?;
        let user = self.repository.registry(hashed_user_registry).await;

        let user = match user {
            RealWorldResult::Ok(user) => {
                info!("[UserRegister] register succeed email is {}", user.email());
                user
            }
            RealWorldResult::Err(err) => {
                error!("Insert failed {}", err);
                return Err(anyhow!("Failed Sign up user."));
            }
        };

        let token = self.jwt_encoder.create_token(&user)?;
        let auth_user = AuthUser::new(user, token);
        RealWorldResult::Ok(auth_user)
    }

    async fn is_exist(&self, email: String) -> RealWorldResult<()> {
        let is_exist = self.repository.is_exist(email.clone()).await?;

        if is_exist == true {
            error!("Already signed email {}", email);
            return Err(anyhow!("Already signed email {}", email));
        } else {
            RealWorldResult::Ok(())
        }
    }

    async fn login(&self, login: UserLogin) -> RealWorldResult<AuthUser> {
        let user = self
            .repository
            .find_by_email(login.email().to_string())
            .await;

        let user = match user {
            Ok(user) => {
                info!("Get User info Id is : {}", user.id());
                user
            }
            Err(_err) => {
                error!("Failed get user info email is {}", login.email());
                return Err(anyhow!("The email does not exist."));
            }
        };

        let verify = self.hash_password.verify(login.password(), user.password());
        if !verify {
            error!("Password is not matched. please check again.");
            return Err(anyhow!("Password is not matched. please check again."));
        }

        let token = self.jwt_encoder.create_token(&user)?;

        RealWorldResult::Ok(AuthUser::new(user, token))
    }

    async fn get_info(&self, id: i64) -> RealWorldResult<AuthUser> {
        let user = self.repository.find_by_id(id).await?;

        let token = self.jwt_encoder.create_token(&user)?;

        RealWorldResult::Ok(AuthUser::new(user, token))
    }

    async fn update(&self, id: i64, request: UserUpdate) -> RealWorldResult<AuthUser> {
        let user = self.repository.find_by_id(id).await?;
        let updated_email = request.email().unwrap_or(user.email()).to_owned();
        let updated_username = request.username().unwrap_or(user.user_name()).to_owned();
        let mut updated_hashed_password = user.password().clone();

        if request.password().is_some() {
            updated_hashed_password = self.hash_password.hash(request.password().unwrap())?;
        }

        let updated_image = if let Some(image) = request.image() {
            Some(image.to_owned())
        } else {
            user.image().to_owned()
        };

        let updated_bio = if let Some(bio) = request.bio() {
            Some(bio.to_owned())
        } else {
            user.bio().to_owned()
        };

        let request = request.update_non_option_fields(
            updated_email,
            updated_username,
            updated_hashed_password,
            updated_image,
            updated_bio,
        );

        let updated_user = self.repository.update(id, request).await?;

        let token = self.jwt_encoder.create_token(&updated_user)?;

        RealWorldResult::Ok(AuthUser::new(updated_user, token))
    }
}
