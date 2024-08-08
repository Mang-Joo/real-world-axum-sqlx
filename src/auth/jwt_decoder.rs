use anyhow::anyhow;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;

use crate::auth::jwt_payload::JwtPayload;

pub struct JwtDecoder {
    secret_key: String,
}

impl JwtDecoder {
    pub fn new(secret_key: String) -> JwtDecoder {
        JwtDecoder { secret_key }
    }
    pub fn decode_token(&self, token: &String) -> anyhow::Result<JwtPayload> {
        let mut validation = Validation::default();
        validation.leeway = 0;

        let token_data = decode::<JwtPayload>(
            token,
            &DecodingKey::from_secret(&self.secret_key.as_bytes()),
            &mut validation,
        );

        match token_data {
            Ok(result) => Ok(result.claims),
            Err(err) => {
                error!("jwt verify error : {err}");
                Err(anyhow!("jwt verify failed {err}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::jwt_encoder::JwtEncoder, user::domain::user::User};

    use super::JwtDecoder;

    const KEY: &str = "secret_key";

    fn fixture_token() -> String {
        let encoder = JwtEncoder::from(String::from(KEY));
        let user = User::new(
            1,
            String::from("email"),
            String::from("passwrod"),
            String::from("username"),
            None,
            None,
        );
        let token = encoder.create_token(&user);
        return token.unwrap();
    }

    #[test]
    fn jwt_decode_test() {
        let token = fixture_token();
        let decoder = JwtDecoder::new(String::from(KEY));

        let jwt_payload = decoder.decode_token(&token);
        assert_eq!(jwt_payload.is_ok(), true);

        let jwt_payload = jwt_payload.unwrap();
        assert_eq!(jwt_payload.id(), 1);
    }
}
