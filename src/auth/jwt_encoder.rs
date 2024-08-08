use std::sync::Arc;

use anyhow::Context;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::auth::clock::RealClock;
use crate::config::RealWorldResult;
use crate::user::domain::user::User;

use super::clock::Clock;
use super::jwt_payload::JwtPayload;

unsafe impl Send for JwtEncoder {}
unsafe impl Sync for JwtEncoder {}

pub type ArcJwtEncoder = Arc<JwtEncoder>;

pub struct JwtEncoder {
    secret_key: String,
    clock: Box<dyn Clock>,
}

impl JwtEncoder {
    pub fn new(secret_key: String, clock: Box<dyn Clock>) -> JwtEncoder {
        JwtEncoder { secret_key, clock }
    }

    pub fn from(secret_key: String) -> JwtEncoder {
        JwtEncoder {
            secret_key,
            clock: Box::new(RealClock),
        }
    }

    pub fn create_token(&self, user: &User) -> RealWorldResult<String> {
        let jwt_payload = JwtPayload::new(user, &self.clock);

        let encoding_key = EncodingKey::from_secret(&self.secret_key.as_bytes());

        encode(&Header::default(), &jwt_payload, &encoding_key).context("create fail jwt token")
    }
}

#[cfg(test)]
mod tests {
    use crate::user::domain::user::User;

    use super::JwtEncoder;

    #[test]
    fn jwt_create_test() {
        let encoder = JwtEncoder::from(String::from("secret_key"));
        let user = User::new(
            1,
            String::from("email"),
            String::from("passwrod"),
            String::from("username"),
            None,
            None,
        );
        let token = encoder.create_token(&user);

        assert_eq!(token.is_ok(), true);
        println!("token : {}", token.unwrap());
    }
}
