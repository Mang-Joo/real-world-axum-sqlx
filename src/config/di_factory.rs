use std::sync::Arc;

use crate::{
    auth::{
        hash_password::{ArgonHash, DynHashPassword, HashPassword},
        jwt_encoder::JwtEncoder,
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
