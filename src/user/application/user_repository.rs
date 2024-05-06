use anyhow::{anyhow, Context};
use chrono::NaiveDateTime;
use log::error;
use sqlx::{Encode, FromRow, Row};

use crate::config;
use crate::config::db::DbPool;
use crate::user::domain::user::User;

pub async fn find_by_email(email: &String, db_pool: &DbPool) -> config::Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT id, email, password, user_name, bio, image, registration_date, modified_date
        FROM users WHERE email = $1 and deleted = false
        "#,
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

pub async fn find_by_user_name(user_name: &String, db_pool: &DbPool) -> config::Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"SELECT id, email, password, user_name, bio, image, registration_date, modified_date
        FROM users WHERE user_name = $1 and deleted = false"#,
        user_name
    ).fetch_optional(db_pool)
        .await
        .context(format!("Failed to find user with name {}", user_name))?;

    let user_entity = match user {
        None => { return Err(anyhow!("Failed to find user with username: {}", user_name)); }
        Some(user) => { user }
    };

    Ok(user_entity.to_domain())
}

pub async fn find_by_id(id: i64, db_pool: &DbPool) -> config::Result<User> {
    let user = sqlx::query_as!(
        UserEntity,
        r#"
        SELECT id, email, password, user_name, bio, image, registration_date, modified_date
        FROM users WHERE id = $1 and deleted = false
        "#,
        id as i32
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

pub async fn update_user_entity(user: &User, db_pool: &DbPool) -> config::Result<()> {
    let entity = UserEntity::from_domain(user);

    let result = sqlx::query(r#"
        UPDATE users
        SET user_name = ?, email = ?, password = ?, bio = ?, image = ?, modified_date = ?
        WHERE id = ?
    "#)
        .bind(&entity.user_name)
        .bind(&entity.email)
        .bind(&entity.password)
        .bind(&entity.bio)
        .bind(&entity.image)
        .bind(&entity.modified_date)
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

pub async fn save_user(user: User, db_pool: &DbPool) -> config::Result<User> {
    let row = sqlx::query(
        "INSERT INTO users (email, password, user_name, bio, image, registration_date, modified_date, deleted)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) returning id
        ")
        .bind(user.email())
        .bind(user.password())
        .bind(user.user_name())
        .bind(user.bio())
        .bind(user.image())
        .bind(user.registration_date().naive_utc())
        .bind(user.modified_date().naive_utc())
        .bind(false)
        .fetch_one(db_pool)
        .await;

    let row = match row {
        Ok(row) => { row }
        Err(err) => {
            error!("err is {err}");
            return Err(anyhow!("Failed save user. User email is {}", user.email()));
        }
    };

    let user = User::new(
        row.get::<i64, usize>(0),
        user.email().to_owned(),
        user.password().to_owned(),
        user.user_name().to_owned(),
        user.bio().to_owned(),
        user.image().to_owned(),
        user.registration_date().to_owned(),
        user.modified_date().to_owned(),
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
    registration_date: NaiveDateTime,
    modified_date: NaiveDateTime,
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
            self.registration_date.and_utc(),
            self.modified_date.and_utc(),
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
            registration_date: domain.registration_date().naive_utc(),
            modified_date: domain.modified_date().naive_utc(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::db::init_db;
    use crate::user::application::user_repository::{find_by_email, find_by_id, update_user_entity};

    #[tokio::test]
    async fn find_email_test() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres")).await;
        let user = find_by_email(&String::from("Hi"), &db)
            .await;

        println!("{:?}", user);

        assert_eq!(user.is_err(), true);
    }

    #[tokio::test]
    async fn update_user_test() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres"))
            .await;
        let user = find_by_id(1, &db)
            .await
            .expect("");

        let user = user.set_user_name(String::from("Update_test"));

        let result = update_user_entity(&user, &db)
            .await
            .expect("Error");

        assert_eq!((), result);
    }
}