use anyhow::anyhow;
use chrono::Duration;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::clock::Clock;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    sub: String,
    exp: usize,
    iat: usize,
}

impl JwtPayload {
    fn new<C: Clock>(user_id: String, clock: &C) -> JwtPayload {
        let now = clock.now()
            .timestamp() as usize;

        let exp = clock.now()
            .checked_add_signed(Duration::hours(3))
            .unwrap()
            .timestamp() as usize;

        JwtPayload {
            sub: user_id,
            exp,
            iat: now,
        }
    }
}

pub async fn encode_jwt<C: Clock>(clock: &C, user_id: &str) -> String {
    let jwt_payload = JwtPayload::new(user_id.to_owned(), clock);

    let encoding_key = &EncodingKey::from_secret("secret".as_ref());

    encode(&Header::default(), &jwt_payload, encoding_key)
        .unwrap()
}

pub async fn verify_jwt<C: Clock>(clock: &C, token: &String) -> anyhow::Result<bool> {
    let validation = Validation::default();
    let token_data = decode::<JwtPayload>(token, &DecodingKey::from_secret("secret".as_ref()), &validation);

    match token_data {
        Ok(result) => Ok(result.claims.exp > clock.now().timestamp() as usize),
        Err(err) => {
            eprintln!("jwt verify error : {err}");
            Err(anyhow!("jwt verify failed"))
        }
    }
}


#[cfg(test)]
mod test {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

    use crate::auth::auth::{encode_jwt, verify_jwt};
    use crate::auth::clock::Clock;

    pub struct MockClock {
        now: DateTime<Utc>,
    }

    impl MockClock {
        pub fn new(now: DateTime<Utc>) -> Self {
            Self { now }
        }
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }
    }

    #[tokio::test]
    async fn encode_jwt_test() {
        let dt: NaiveDateTime =
            NaiveDate::from_ymd_opt(2024, 4, 7)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap();

        let clock = MockClock::new(DateTime::from_naive_utc_and_offset(dt, Utc));

        let token = encode_jwt(&clock, "hello")
            .await;

        assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsbyIsImV4cCI6MTcxMjQ5MTgxMSwiaWF0IjoxNzEyNDgxMDExfQ.YzWeIIXoSQPnwB6oYS6rkasivXgSUVkEIvuXcbp_gnw");
    }

    #[tokio::test]
    async fn decode_jwt_test() {
        let dt: NaiveDateTime =
            NaiveDate::from_ymd_opt(2024, 4, 7)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap();

        let clock = MockClock::new(DateTime::from_naive_utc_and_offset(dt, Utc));

        let verified = verify_jwt(
            &clock,
            &String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsbyIsImV4cCI6MTcxMjQ5MTgxMSwiaWF0IjoxNzEyNDgxMDExfQ.YzWeIIXoSQPnwB6oYS6rkasivXgSUVkEIvuXcbp_gnw"),
        ).await
            .is_err();

        assert_eq!(verified, true);
    }
}