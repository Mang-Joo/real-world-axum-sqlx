use anyhow::{anyhow, Ok};
use axum::async_trait;
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

use crate::{
    config::{db::DbPool, RealWorldResult},
    user::{
        domain::{repository::UserRepository, user::User},
        service::model::UserRegistry,
    },
};

pub struct ConcreteUserRepository {
    db_pool: DbPool,
}

impl ConcreteUserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for ConcreteUserRepository {
    async fn registry(&self, user_register: UserRegistry) -> RealWorldResult<User> {
        let result = sqlx::query_as!(
            UserEntity,
            "INSERT INTO users (email, username, password)
            VALUES ($1, $2, $3)
            RETURNING *
             ",
            user_register.email(),
            user_register.user_name(),
            user_register.password()
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|err| anyhow!("User Insert Failed {}", err))?;

        Ok(result.to_user())
    }

    async fn is_exist(&self, user_email: String) -> RealWorldResult<bool> {
        let result = sqlx::query!(
            r#"
        SELECT 1 AS exists 
        FROM users
        WHERE email = $1
        "#,
            user_email
        )
        .fetch_optional(&self.db_pool)
        .await?;

        RealWorldResult::Ok(result.is_some())
    }
}

#[derive(FromRow)]
struct UserEntity {
    id: i64,
    email: String,
    username: String,
    password: String,
    bio: Option<String>,
    image: Option<String>,
    registration_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    deleted: bool,
}

impl UserEntity {
    fn to_user(self) -> User {
        User::new(
            self.id,
            self.email,
            self.password,
            self.username,
            self.bio,
            self.image,
        )
    }
}
