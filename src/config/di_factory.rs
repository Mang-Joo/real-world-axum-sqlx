use std::sync::Arc;

use crate::{
    auth::{
        hash_password::{ArgonHash, DynHashPassword},
        jwt_encoder::JwtEncoder,
    },
    profile::{
        domain::service::DynProfileService, repository::repository::ConcreteProfileRepository,
        service::service::ConcreteProfileService,
    },
    user::{
        domain::{repository::DynUserRepository, service::DynUserService},
        repository::repository::ConcreteUserRepository,
        service::service::ConcreteUserService,
    },
};

use super::{app_state::ArcAppState, db::DbPool};

pub fn create_user_service(db_pool: DbPool, arc_app_state: ArcAppState) -> DynUserService {
    let hash_password: DynHashPassword = Arc::new(ArgonHash::default());
    let concrete_user_repository: DynUserRepository =
        Arc::new(ConcreteUserRepository::new(db_pool.clone()));
    let jwt_encoder = JwtEncoder::from(arc_app_state.secret_key.clone());

    let user_service: DynUserService = Arc::new(ConcreteUserService::new(
        concrete_user_repository,
        hash_password,
        Arc::new(jwt_encoder),
    ));
    user_service
}

pub fn create_profile_service(db_pool: DbPool, user_service: DynUserService) -> DynProfileService {
    let user_service = user_service;
    let repository = ConcreteProfileRepository::new(db_pool.clone());
    let repository = Arc::new(repository);

    let profile_service = ConcreteProfileService::new(repository, user_service);
    Arc::new(profile_service)
}
