use anyhow::{anyhow, Context};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::async_trait;

#[async_trait]
pub trait HashPassword {
    async fn hash(&self, plain_data: &String) -> anyhow::Result<String>;
    async fn verify(&self, plain_data: String, hashed_data: &String) -> bool;
}

pub struct ArgonHash;

impl ArgonHash {
    pub fn default() -> ArgonHash {
        ArgonHash
    }
}

#[async_trait]
impl HashPassword for ArgonHash {
    async fn hash(&self, plain_data: &String) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let config = Argon2::default();

        let hashed_data = match config.hash_password(plain_data.as_bytes(), &salt) {
            Ok(hashed_data) => hashed_data.to_string(),
            Err(_) => { return Err(anyhow!("Failed data hashing")); }
        };

        Ok(hashed_data)
    }

    async fn verify(&self, plain_data: String, hashed_data: &String) -> bool {
        let argon2 = Argon2::default();

        let hash = PasswordHash::new(&hashed_data)
            .unwrap();

        argon2.verify_password(plain_data.as_bytes(), &hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::user::domain::hash_password::{ArgonHash, HashPassword};

    #[tokio::test]
    async fn hash_test() {
        let hello = String::from("Hello");
        let hashed = ArgonHash::default().hash(&hello)
            .await
            .unwrap();

        assert_ne!(hello, hashed);
    }

    #[tokio::test]
    async fn verify_test() {
        let hello = String::from("hello");
        let hashed_data = String::from("$argon2id$v=19$m=19456,t=2,p=1$ULFfcwnYvCZwgiRm1i97yg$OcMjE44RqEVd4fzKFUJtuBJMsVEvQX2641nYX9ZCQDY");

        let response = ArgonHash::default().verify(hello, &hashed_data).await;

        assert_eq!(response, false);
    }

    #[tokio::test]
    async fn wrong_hashed_data_verify_test() {
        let hello = String::from("hello");
        let hashed_data = String::from("=19$m=19456,t=2,p=1$ULFfcwnYvCZwgiRm1i97yg$OcMjE44RqEVd4fzKFUJtuBJMsVEvQX2641nYX9ZCQDY");

        let response = ArgonHash::default().verify(hello, &hashed_data);

        assert_eq!(response, false);
    }
}