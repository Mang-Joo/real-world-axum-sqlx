use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::user::domain::user::User;

use super::clock::Clock;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    id: i64,
    sub: String,
    exp: usize,
    iat: usize,
}

impl JwtPayload {
    pub fn new(user: &User, clock: &Box<dyn Clock>) -> JwtPayload {
        let now = clock.now().timestamp() as usize;

        let expired_at = clock
            .now()
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

#[cfg(test)]
mod tests {
    use crate::{
        auth::clock::{Clock, RealClock},
        user::domain::user::User,
    };

    use super::JwtPayload;

    #[test]
    fn create_payload_test() {
        let user = User::new(
            1,
            String::from("email"),
            String::from("password"),
            String::from("username"),
            None,
            None,
        );
        let real_clock: Box<dyn Clock> = Box::new(RealClock);
        let jwt_payload = JwtPayload::new(&user, &real_clock);

        assert_eq!(jwt_payload.id, 1);
    }
}
