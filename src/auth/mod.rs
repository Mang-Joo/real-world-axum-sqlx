use crate::auth::clock::Clock;

pub mod auth;
mod clock;
mod auth_tests;
pub mod jwt_encoder;
pub mod jwt_decoder;

type JwtClock = dyn Clock + 'static + Send + Sync;
