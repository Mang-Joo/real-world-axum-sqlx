use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::auth::clock::Clock;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    sub: String,
    exp: usize,
    iat: usize,
}

impl JwtPayload {
    pub fn new(user_id: String, clock: &Box<dyn Clock>) -> JwtPayload {
        let now = clock.now()
            .timestamp() as usize;

        let expired_at = clock.now()
            .checked_add_signed(Duration::hours(3))
            .unwrap()
            .timestamp() as usize;

        JwtPayload {
            sub: user_id,
            exp: expired_at,
            iat: now,
        }
    }

    pub fn user_id(self) -> String {
        self.sub
    }
}