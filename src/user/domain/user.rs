use anyhow::Context;
use chrono::{DateTime, Utc};
use crate::config;

use crate::user::domain::hash_password::HashPassword;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    email: String,
    password: String,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
    registration_date: DateTime<Utc>,
    modified_date: DateTime<Utc>,
}

unsafe impl Send for User {}

unsafe impl Sync for User {}

impl User {
    pub fn new(
        id: i64,
        email: String,
        password: String,
        user_name: String,
        bio: Option<String>,
        image: Option<String>,
        registration_date: DateTime<Utc>,
        modified_date: DateTime<Utc>,
    ) -> User {
        User {
            id,
            email,
            password,
            user_name,
            bio,
            image,
            registration_date,
            modified_date,
        }
    }

    pub async fn not_verify_password(&self, input_password: String, hash: &(dyn HashPassword + Send + Sync)) -> bool {
        !hash.verify(input_password, &self.password)
            .await
    }

    pub async fn hash_password(self, hash: &(dyn HashPassword + Send + Sync)) -> config::Result<User> {
        let hashed_password = hash
            .hash(&self.password)
            .await
            .context("Failed hashing password")?;

        let user = User {
            password: hashed_password,
            ..self
        };
        Ok(user)
    }


    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password(&self) -> &String {
        &self.password
    }
    pub fn user_name(&self) -> &String {
        &self.user_name
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn image(&self) -> &Option<String> {
        &self.image
    }


    pub fn set_email(self, email: String) -> User {
        User {
            email,
            modified_date: Utc::now(),
            ..self
        }
    }
    pub fn set_password(self, password: String) -> User {
        User {
            password,
            modified_date: Utc::now(),
            ..self
        }
    }
    pub fn set_user_name(self, user_name: String) -> User {
        User {
            user_name,
            modified_date: Utc::now(),
            ..self
        }
    }
    pub fn set_bio(self, bio: Option<String>) -> User {
        User {
            bio,
            modified_date: Utc::now(),
            ..self
        }
    }
    pub fn set_image(self, image: Option<String>) -> Self {
        User {
            image,
            modified_date: Utc::now(),
            ..self
        }
    }
    pub fn registration_date(&self) -> DateTime<Utc> {
        self.registration_date
    }
    pub fn modified_date(&self) -> DateTime<Utc> {
        self.modified_date
    }
}