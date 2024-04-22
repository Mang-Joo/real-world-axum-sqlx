use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::auth::JwtClock;
use crate::user::domain::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    id: i64,
    sub: String,
    exp: usize,
    iat: usize,
}

impl JwtPayload {
    pub fn new(user: &User, clock: &Box<JwtClock>) -> JwtPayload {
        let now = clock.now()
            .timestamp() as usize;

        let expired_at = clock.now()
            .checked_add_signed(Duration::hours(3))
            .unwrap()
            .timestamp() as usize;

        JwtPayload {
            id: user.id(),
            sub: user.email().to_string(),
            exp: expired_at,
            iat: now,
        }
    }


    pub fn id(&self) -> i64 {
        self.id
    }
}