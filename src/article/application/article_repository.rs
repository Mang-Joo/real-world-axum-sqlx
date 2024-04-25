use anyhow::anyhow;
use chrono::{DateTime, Utc};
use log::{error, info};
use sqlx::{MySql, Transaction};

use tag_repository::save_tags;

use crate::article::application::article_tag_repository::save_article_and_tags;
use crate::article::application::tag_repository;
use crate::article::domain::article::Article;
use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::db::DbPool;
use crate::user::domain::user::User;

pub async fn save_article(
    article: Article,
    db_pool: &DbPool,
) -> config::Result<Article> {

    let mut transaction: Transaction<'_, MySql> = db_pool.begin().await.unwrap();

    let result = sqlx::query(r#"
        INSERT INTO article (title, description, body, created_at, updated_at, user_id)
        VALUES (?, ?, ?, ?, ?, ?)
    "#)
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

struct ArticleEntity {
    id: i64,
    user_id: i64,
    slug: String,
    title: String,
    description: String,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ArticleEntity {
    fn from_domain(article: Article) -> Self {
        ArticleEntity {
            id: article.id(),
            slug: article.slug().to_string(),
            title: article.title().to_owned(),
            description: article.description().to_owned(),
            body: article.body().to_owned(),
            updated_at: article.updated_at(),
            created_at: article.created_at(),
            user_id: article.author().id(),
        }
    }

    fn to_domain(
        self,
        user: User,
        tag_list: Option<Vec<Tag>>,
    ) -> Article {
        Article::new(
            self.id,
            self.title,
            self.description,
            self.body,
            tag_list,
            user,
        )
    }
}

