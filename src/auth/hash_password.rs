use std::sync::Arc;

use anyhow::anyhow;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::config::{self, RealWorldResult};

pub type DynHashPassword = Arc<dyn HashPassword + Send + Sync>;

pub trait HashPassword {
    fn hash(&self, plain_data: &String) -> RealWorldResult<String>;
    fn verify(&self, plain_data: String, hashed_data: &String) -> bool;
}

#[derive(Default)]
pub struct ArgonHash;

impl HashPassword for ArgonHash {
    fn hash(&self, plain_data: &String) -> config::RealWorldResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let config = Argon2::default();

        let hashed_data = match config.hash_password(plain_data.as_bytes(), &salt) {
            Ok(hashed_data) => hashed_data.to_string(),
            Err(_) => {
                return Err(anyhow!("Failed data hashing"));
            }
        };

        Ok(hashed_data)
    }

    fn verify(&self, plain_data: String, hashed_data: &String) -> bool {
        let argon2 = Argon2::default();

        let hash = PasswordHash::new(hashed_data);

        let hash = match hash {
            Ok(hash) => hash,
            Err(err) => {
                eprintln!("Error {:?}", err);
                return false;
            }
        };

        argon2.verify_password(plain_data.as_bytes(), &hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::auth::hash_password::{ArgonHash, HashPassword};

    #[test]
    fn hash_test() {
        let hello = String::from("Hello");
        let hashed = ArgonHash::default().hash(&hello).unwrap();

        assert_ne!(hello, hashed);
    }

    #[test]
    fn verify_test() {
        let hello = String::from("hello");

        let hashed_data = String::from("$argon2id$v=19$m=19456,t=2,p=1$ULFfcwnYvCZwgiRm1i97yg$OcMjE44RqEVd4fzKFUJtuBJMsVEvQX2641nYX9ZCQDY");

        let response = ArgonHash::default().verify(hello, &hashed_data);

        assert_eq!(response, false);
    }

    #[test]
    fn wrong_hashed_data_verify_test() {
        let hello = String::from("hello");
        let hashed_data = Arc::new(String::from("$argon2id$v=19$m=19456,t=2,p=1$ULFfcwnYvCZwgiRm1i97yg$OcMjE44RqEVd4fzKFUJtuBJMsVEvQX2641nYX9ZCQDA"));

        let response = ArgonHash::default().verify(hello, &hashed_data);

        assert_eq!(response, false);
    }
}
