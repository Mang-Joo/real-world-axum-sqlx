use anyhow::anyhow;
use axum::async_trait;
use log::{error, info};

use crate::{
    auth::{hash_password::DynHashPassword, jwt_encoder::ArcJwtEncoder},
    config::RealWorldResult,
    user::domain::{
        model::{UserLogin, UserRegistry},
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
            Ok(())
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

        Ok(AuthUser::new(user, token))
    }
}
