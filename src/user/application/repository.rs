use anyhow::{anyhow, Context};
use log::error;

use crate::app_state::Result;
use crate::db::DbPool;
use crate::user::domain::user::User;

pub async fn find_by_email(email: &String, db_pool: &DbPool) -> Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, email, password, user_name, bio, image FROM users WHERE email = ?",
        email
    ).fetch_optional(db_pool)
        .await
        .context(format!("Failed to find user with email: {}", email))?;

    let user_entity = match user {
        None => { return Err(anyhow!("Failed to find user with email: {}", email)); }
        Some(user) => { user }
    };

    Ok(user_entity.to_domain())
}

pub async fn find_by_id(id: i64, db_pool: &DbPool) -> Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, email, password, user_name, bio, image FROM users WHERE id = ?",
        id
    ).fetch_optional(db_pool)
        .await
        .context(format!("Failed to find user with email: {}", id))?;

    let user_entity = match user {
        None => {
            error!("Failed find user id {}", id);
            return Err(anyhow!("Failed to find user with id: {}", id));
        }
        Some(user) => { user }
    };

    Ok(user_entity.to_domain())
}

pub async fn save_user(user: User, db_pool: &DbPool) -> Result<User> {
    let row = sqlx::query(
        "INSERT INTO users (email, password, user_name, bio, image)
         VALUES (?, ?, ?, ?, ?)
        ")
        .bind(user.email())
        .bind(user.password())
        .bind(user.user_name())
        .bind(user.bio())
        .bind(user.image())
        .execute(db_pool)
        .await;

    let row = match row {
        Ok(row) => { row }
        Err(err) => {
            println!("{err}");
            error!("err is {err}");
            return Err(anyhow!("Save Failed user. User id is {}", user.email()));
        }
    };

    let user = User::new(
        row.last_insert_id() as i64,
        user.email().to_owned(),
        user.password().to_owned(),
        user.user_name().to_owned(),
        user.bio().to_owned(),
        user.image().to_owned(),
    );

    Ok(user)
}

struct UserEntity {
    id: i64,
    email: String,
    password: String,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
}

impl UserEntity {
    fn to_domain(self) -> User {
        User::new(
            self.id,
            self.email,
            self.password,
            self.user_name,
            self.bio,
            self.image,
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::user::application::repository::find_by_email;

    #[tokio::test]
    async fn find_email_test() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world")).await;
        let user = find_by_email(&String::from("Hi"), &db)
            .await;

        println!("{:?}", user);

        assert_eq!(user.is_err(), true);
    }
}