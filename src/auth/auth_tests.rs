#[cfg(test)]
mod test {
    use std::sync::Arc;

    use anyhow::anyhow;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

    use crate::app_state::init_app_state;
    use crate::auth::clock::Clock;
    use crate::auth::jwt_decoder::JwtDecoder;
    use crate::auth::jwt_encoder::JwtEncoder;

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
        let app_state = init_app_state().await;
        let app_state = Arc::new(app_state);

        let dt: NaiveDateTime =
            NaiveDate::from_ymd_opt(2024, 4, 7)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap();

        let clock = MockClock::new(DateTime::from_naive_utc_and_offset(dt, Utc));

        let jwt_encoder = JwtEncoder::new(app_state, Box::new(clock));

        let token = jwt_encoder
            .encode_jwt("hello")
            .await
            .map_err(|err| anyhow!(err));

        // assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsbyIsImV4cCI6MTcxMjQ5MTgxMSwiaWF0IjoxNzEyNDgxMDExfQ.YzWeIIXoSQPnwB6oYS6rkasivXgSUVkEIvuXcbp_gnw");
    }

    #[tokio::test]
    async fn decode_jwt_test() {
        let dt: NaiveDateTime =
            NaiveDate::from_ymd_opt(2024, 4, 7)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap();

        let clock = MockClock::new(DateTime::from_naive_utc_and_offset(dt, Utc));

        let app_state = init_app_state().await;
        let app_state = Arc::new(app_state);
        let jwt_decoder = JwtDecoder::new(app_state);

        let verified = jwt_decoder.verify_jwt(
            &String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsbyIsImV4cCI6MTcxMjQ5MTgxMSwiaWF0IjoxNzEyNDgxMDExfQ.YzWeIIXoSQPnwB6oYS6rkasivXgSUVkEIvuXcbp_gnw"),
        ).await
            .map_err(|err| println!("error : {err}"))
            .is_err();

        assert_eq!(verified, true);
    }
}