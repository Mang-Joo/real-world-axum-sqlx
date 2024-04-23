use std::sync::Arc;

use anyhow::anyhow;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;

use crate::auth::auth::JwtPayload;
use crate::config;
use crate::config::app_state::AppState;

pub struct JwtDecoder {
    app_state: Arc<AppState>,
}

impl JwtDecoder {
    pub fn new(app_state: Arc<AppState>) -> JwtDecoder {
        JwtDecoder {
            app_state,
        }
    }
    pub async fn decode_token(&self, token: &String) -> config::Result<JwtPayload> {
        let mut validation = Validation::default();
        validation.leeway = 0;

        let token_data = decode::<JwtPayload>(token, &DecodingKey::from_secret(&self.app_state.secret_key.as_ref()), &validation);

        match token_data {
            Ok(result) => Ok(result.claims),
            Err(err) => {
                error!("jwt verify error : {err}");
                Err(anyhow!("jwt verify failed {err}"))
            }
        }
    }
}