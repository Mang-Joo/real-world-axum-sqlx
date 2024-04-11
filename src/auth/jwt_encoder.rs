use std::sync::Arc;

use anyhow::Context;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::app_state::AppState;
use crate::auth::auth::JwtPayload;
use crate::auth::clock::RealClock;
use crate::auth::JwtClock;

pub struct JwtEncoder {
    app_state: Arc<AppState>,
    clock: Box<JwtClock>,
}

impl JwtEncoder {
    pub fn new(app_state: Arc<AppState>, clock: Box<JwtClock>) -> JwtEncoder {
        JwtEncoder {
            app_state,
            clock,
        }
    }

    pub fn from(app_state: Arc<AppState>) -> JwtEncoder {
        JwtEncoder {
            app_state,
            clock: Box::new(RealClock),
        }
    }

    pub async fn encode_jwt(&self, user_id: &str) -> anyhow::Result<String> {
        let jwt_payload = JwtPayload::new(user_id.to_owned(), &self.clock);

        let encoding_key = &EncodingKey::from_secret(&self.app_state.secret_key.as_ref());

        encode(&Header::default(), &jwt_payload, encoding_key)
            .context("create fail jwt token")
    }
}