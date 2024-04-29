use anyhow::{anyhow, Context};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info};
use sqlx::{Encode, FromRow, MySql, Transaction, Type};

use tag_repository::save_tags;

use crate::article::application::article_tag_repository::save_article_and_tags;
use crate::article::application::tag_repository;
use crate::article::domain::article::Article;
use crate::config;
use crate::config::db::DbPool;
use crate::config::error::AppError;

pub async fn save_article(
    article: Article,
    db_pool: &DbPool,
) -> config::Result<Article> {
    let mut transaction: Transaction<'_, MySql> = db_pool.begin().await.unwrap();

    let result = sqlx::query(r#"
        INSERT INTO article (slug, title, description, body, created_at, updated_at, user_id)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    "#)
        .bind(article.slug())
        .bind(article.title())
        .bind(article.description())
        .bind(article.body())
        .bind(article.created_at())
        .bind(article.updated_at())
        .bind(article.author().id())
        .execute(&mut *transaction)
        .await?;

    let inserted_id = result.last_insert_id() as i64;

    let tags = if let Some(tags) = article.tag_list() {
        info!("tags {:?}", tags);

        let tags = save_tags(tags, &mut transaction).await?;

        save_article_and_tags(inserted_id, &tags, &mut transaction).await?;
        Some(tags)
    } else { None };

    let _ = transaction
        .commit()
        .await
        .map_err(|err| {
            error!("Transaction commit error: {}", err);
            anyhow!("Transaction failed")
        });


    let article = Article::new(
        inserted_id as i64,
        article.title().to_owned(),
        article.description().to_owned(),
        article.body().to_owned(),
        tags,
        article.author().to_owned(),
    );

    Ok(article)
}

pub async fn get_single_article_by_repository(
    slug: String,
    db_pool: &DbPool,
) -> config::Result<Article> {
    let article_author_entity = sqlx::query_as!(
        ArticleAndAuthorEntity,
        "SELECT article.id as article_id,
       article.slug,
       article.title,
       article.description,
       article.body,
       article.created_at,
       article.updated_at,
       author.id    as user_id,
       author.user_name,
       author.bio,
       author.image
FROM article
         JOIN users author on article.user_id = author.id
WHERE article.slug = ?
  and article.deleted = false",
        slug
    ).fetch_one(db_pool)
        .await
        .context(format!("Did not find slug {}", slug))?;


    todo!()
}

#[derive(Debug, FromRow, Encode, Type)]
struct ArticleAndAuthorEntity {
    article_id: i64,
    slug: String,
    title: String,
    description: String,
    body: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    user_id: i64,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
}

impl ArticleAndAuthorEntity {}

mod tests {
    use crate::article::application::article_repository::get_single_article_by_repository;
    use crate::config::db::init_db;

    #[tokio::test]
    async fn get_single_article() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world")).await;

        let article = get_single_article_by_repository(
            String::from("Hello-mangjoo-"),
            &db,
        ).await;

        assert_eq!(article.is_ok(), true);
    }
}