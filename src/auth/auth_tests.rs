#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::time::Duration;

    use anyhow::anyhow;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
    use tokio::time::sleep;

    use crate::app_state::init_app_state;
    use crate::auth::clock::{Clock, RealClock};
    use crate::auth::jwt_decoder::JwtDecoder;
    use crate::auth::jwt_encoder::JwtEncoder;
    use crate::user::domain::user::User;

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

        let user = User::new(
            5,
            String::from("Hello"),
            String::new(),
            String::new(),
            Some(String::new()),
            Some(String::new()),
        );

        let token = jwt_encoder
            .encode_jwt(&user)
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

        tokio::time::sleep(Duration::from_secs(5)).await;

        let verified = jwt_decoder.decode_token(
            &String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoZWxsbyIsImV4cCI6MTcxMjQ5MTgxMSwiaWF0IjoxNzEyNDgxMDExfQ.YzWeIIXoSQPnwB6oYS6rkasivXgSUVkEIvuXcbp_gnw"),
        ).await
            .map_err(|err| println!("error : {err}"))
            .is_err();

        assert_eq!(verified, true);
    }

    #[tokio::test]
    #[should_panic(expected = "jwt verify failed ExpiredSignature")]
    async fn jwt_encode_decode_error_test() {
        let app_state = init_app_state().await;
        let app_state = Arc::new(app_state);
        let encoder = JwtEncoder::from(app_state);

        let user = User::new(
            5,
            String::from("Hello"),
            String::new(),
            String::new(),
            Some(String::new()),
            Some(String::new()),
        );

        let jwt = encoder.encode_jwt(&user).await;

        let jwt = match jwt {
            Ok(jwt) => { jwt }
            Err(_) => { panic!("") }
        };
        let clock = RealClock;
        let now = RealClock::now(&clock);
        println!("before now : {now}");
        let sleep = sleep(Duration::from_secs(2)).await;
        let now = RealClock::now(&clock);
        println!("after now : {now}");

        let app_state = init_app_state().await;
        let app_state = Arc::new(app_state);
        let jwt_decoder = JwtDecoder::new(app_state);

        let code = jwt_decoder.decode_token(&jwt).await;
        match code {
            Ok(_) => {}
            Err(err) => { panic!("{}", err) }
        }
    }
}