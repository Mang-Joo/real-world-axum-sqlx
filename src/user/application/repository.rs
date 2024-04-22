use anyhow::{anyhow, Context};
use chrono::{DateTime, TimeZone, Utc};
use log::error;
use sqlx::{Encode, FromRow};

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

pub async fn find_by_user_name(user_name: &String, db_pool: &DbPool) -> Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        "SELECT id, email, password, user_name, bio, image FROM users WHERE user_name = ?",
        user_name
    ).fetch_optional(db_pool)
        .await
        .context(format!("Failed to find user with name {}", user_name))?;

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

pub async fn update_user_entity(user: &User, db_pool: &DbPool) -> Result<()> {
    let entity = UserEntity::from_domain(user);

    let result = sqlx::query(r#"
        UPDATE users
        SET user_name = ?, email = ?,
            password = ?, bio = ?, image = ?
        WHERE id = ?
    "#)
        .bind(&entity.user_name)
        .bind(&entity.email)
        .bind(&entity.password)
        .bind(&entity.bio)
        .bind(&entity.image)
        .bind(entity.id)
        .execute(db_pool)
        .await;

    match result {
        Ok(_) => { () }
        Err(err) => {
            error!("err is {err}");
            eprintln!("err is {err}");
            return Err(anyhow!("Failed update user. User email is {}", user.email()));
        }
    };


    Ok(())
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
            error!("err is {err}");
            return Err(anyhow!("Failed save user. User email is {}", user.email()));
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

#[derive(FromRow, Encode)]
struct UserEntity {
    id: i64,
    email: String,
    password: String,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
    registration_date: DateTime<Utc>,
    modified_date: DateTime<Utc>,
    deleted: bool
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

    fn from_domain(domain: &User) -> Self {
        UserEntity {
            id: domain.id(),
            user_name: domain.user_name().to_owned(),
            email: domain.email().to_owned(),
            password: domain.password().to_owned(),
            bio: domain.bio().to_owned(),
            image: domain.image().to_owned(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::user::application::repository::{find_by_email, find_by_id, update_user_entity};

    #[tokio::test]
    async fn find_email_test() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world")).await;
        let user = find_by_email(&String::from("Hi"), &db)
            .await;

        println!("{:?}", user);

        assert_eq!(user.is_err(), true);
    }

    #[tokio::test]
    async fn update_user_test() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world")).await;
        let user = find_by_id(1, &db)
            .await
            .expect("");

        let user = user.set_user_name(String::from("Update_test"));

        let result = update_user_entity(&user, &db).await
            .expect("Error");

        assert_eq!((), result);
    }
}