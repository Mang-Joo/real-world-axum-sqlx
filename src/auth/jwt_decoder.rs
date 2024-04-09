use std::sync::Arc;

use anyhow::anyhow;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::app_state::AppState;
use crate::auth::auth::JwtPayload;

pub struct JwtDecoder {
    app_state: Arc<AppState>,
}

impl JwtDecoder {
    pub fn new(app_state: Arc<AppState>) -> JwtDecoder {
        JwtDecoder {
            app_state,
        }
    }
    pub async fn verify_jwt(&self, token: &String) -> anyhow::Result<String> {
        let validation = Validation::default();
        let token_data = decode::<JwtPayload>(token, &DecodingKey::from_secret(&self.app_state.secret_key.as_ref()), &validation);

        match token_data {
            Ok(result) => Ok(result.claims.user_id()),
            Err(err) => {
                eprintln!("jwt verify error : {err}");
                Err(anyhow!("jwt verify failed {err}"))
            }
        }
    }
}