use std::fmt::format;

use anyhow::{anyhow, Context};
use chrono::NaiveDateTime;
use log::{error, info};
use sqlx::{Encode, Execute, Postgres, QueryBuilder, Row, Transaction};
use sqlx::FromRow;
use sqlx::query::Query;

use tag_repository::save_tags;

use crate::article::application::{article_tag_repository, tag_repository};
use crate::article::application::article_favorite_repository::count_favorite_by_article_id;
use crate::article::application::article_tag_repository::save_article_and_tags;
use crate::article::application::get_articles_default_usecase::ListArticleRequest;
use crate::article::domain::article::{Article, ArticleWithFavorite, Author};
use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::db::DbPool;
use crate::user::application::get_current_user_usecase::get_current_user;
use crate::user::application::user_repository::find_by_user_name;

pub async fn save_article(
    article: Article,
    db_pool: &DbPool,
) -> config::Result<Article> {
    let mut transaction: Transaction<'_, Postgres> = db_pool.begin().await.unwrap();

    let result = sqlx::query(r#"
        INSERT INTO article (slug, title, description, body, created_at, updated_at, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7) returning id
    "#)
        .bind(article.slug())
        .bind(article.title())
        .bind(article.description())
        .bind(article.body())
        .bind(article.created_at())
        .bind(article.updated_at())
        .bind(article.author().id())
        .fetch_one(&mut *transaction)
        .await?;

    let inserted_id = result.get::<i64, usize>(0);

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
        0,
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
from article
        join users author on article.user_id = author.id
where article.slug = $1
and article.deleted = false",
        slug
    ).fetch_one(db_pool)
        .await
        .context(format!("Did not find slug {}", slug))?;

    let tags = article_tag_repository::get_tags_by_article_id(
        article_author_entity.article_id,
        db_pool,
    ).await?;

    let favorite_count = count_favorite_by_article_id(
        article_author_entity.article_id,
        db_pool,
    ).await?;

    let article = article_author_entity.to_domain(tags, favorite_count);

    Ok(article)
}

pub async fn get_default_articles_by_repository(
    article_query: ListArticleRequest,
    db_pool: &DbPool,
) -> config::Result<Vec<Article>> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"
SELECT article.id as article_id,
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
JOIN article_tag tag on article.id = tag.article_id
WHERE article.deleted = false
    "#);


    if let Some(tag) = article_query.tag() {
        println!("tag {tag}");
        query_builder
            .push("AND tag.tag_name = ")
            .push_bind(tag);
    };

    if let Some(author) = article_query.author() {
        query_builder
            .push("AND author.user_name = ")
            .push_bind(author);
    };

    if let Some(favorite_user) = article_query.favorited() {
        let user = find_by_user_name(favorite_user, db_pool)
            .await?;
        query_builder
            .push("AND user_id = ")
            .push_bind(user.id());
    };

    query_builder
        .push(" LIMIT ")
        .push_bind(article_query.limit().unwrap_or(20))
        .push(" OFFSET ")
        .push_bind(article_query.offset().unwrap_or(0));

    let result = query_builder
        .build_query_as::<ArticleAndAuthorEntity>()
        .fetch_all(db_pool)
        .await
        .map_err(|err| {
            error!("Failed get articles Error : {}", err.to_string());
            anyhow!("Failed get articles")
        })?;

    Err(anyhow!("fawjop"))
}

#[derive(Debug, FromRow, Encode)]
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

impl ArticleAndAuthorEntity {
    fn to_domain(
        self,
        tags: Option<Vec<Tag>>,
        favorite_count: i64,
    ) -> Article {
        let author = Author::new(
            self.user_id,
            self.user_name,
            self.bio,
            self.image,
        );
        Article::new(
            self.article_id,
            self.title,
            self.description,
            self.body,
            favorite_count,
            tags,
            author,
        )
    }
}

mod tests {
    use crate::article::application::article_repository::{get_default_articles_by_repository, get_single_article_by_repository};
    use crate::article::application::get_articles_default_usecase::ListArticleRequest;
    use crate::config::db::init_db;

    #[tokio::test]
    async fn get_single_article() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres")).await;

        let article = get_single_article_by_repository(
            String::from("Hello-mangjoo-"),
            &db,
        ).await;

        assert_eq!(article.is_ok(), true);
    }

    #[tokio::test]
    async fn get_list_article() {
        let db = init_db(String::from("postgresql://postgres:11223344@146.56.115.136:5432/postgres")).await;

        let request = ListArticleRequest::new(
            Some(String::from("nooo")),
            None,
            None,
            None,
            None,
        );

        let result = get_default_articles_by_repository(
            request,
            &db,
        ).await.is_err();
    }
}