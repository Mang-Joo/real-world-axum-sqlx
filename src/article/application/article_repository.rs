use std::sync::Arc;

use anyhow::anyhow;
use chrono::NaiveDateTime;
use log::{error, info};
use sqlx::{Encode, Postgres, Row, Transaction};
use sqlx::FromRow;

use crate::article::application::article_tag_repository::save_article_and_tags;
use crate::article::application::get_articles_default_usecase::ListArticleRequest;
use crate::article::application::tag_repository::save_tags;
use crate::article::domain::article::{Article, ArticleWithFavorite, Author};
use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::app_state::{AppState, ArcAppState};
use crate::user::application::user_repository::find_by_user_name;

pub async fn save_article(article: Article, app_state: ArcAppState) -> config::Result<Article> {
    let db_pool = &app_state.pool;
    let mut transaction: Transaction<'_, Postgres> = db_pool.begin().await.unwrap();

    let result = sqlx::query(
        r#"
        INSERT INTO article (slug, title, description, body, created_at, updated_at, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7) returning id
    "#,
    )
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
    } else {
        None
    };

    let _ = transaction.commit().await.map_err(|err| {
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
    app_state: ArcAppState,
) -> config::Result<Article> {
    let article_author_entity = sqlx::query_as!(
        SingleArticleEntity,
        "
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
            author.image,
            array_agg(tag.tag_name)    tags,
            COUNT(favorite.article_id) favorite_count
        from article
            join users author on article.user_id = author.id
            LEFT JOIN article_tag tag on article.id = tag.article_id
            LEFT JOIN article_favorite favorite on article.id = favorite.article_id
        where article.slug = $1
            and article.deleted = false
        GROUP BY article.id, author.id, created_at
",
        slug
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|err| {
        println!("{}", err.to_string());
        anyhow!(format!("Did not find slug {}", slug))
    })?;

    if let Some(article_entity) = article_author_entity {
        info!("Find success entity {:?}", article_entity);
        Ok(article_entity.to_domain())
    } else {
        error!("Failed get article {}", slug);
        Err(anyhow!("Failed find article"))
    }
}

pub async fn get_default_articles_by_repository(
    user_id: Option<i64>,
    article_query: ListArticleRequest,
    app_state: ArcAppState,
) -> config::Result<Vec<ArticleWithFavorite>> {
    let favorite_id = if let Some(favorite_user) = article_query.favorited() {
        let user = find_by_user_name(favorite_user, Arc::clone(&app_state)).await?;
        Some(user.id())
    } else {
        None
    };

    let result = sqlx::query_as!(
        MultipleArticleEntity,
        r#"
SELECT article.id                         as article_id,
       article.slug,
       article.title,
       article.description,
       article.body,
       article.created_at,
       article.updated_at,
       author.id                          as user_id,
       author.user_name,
       author.bio,
       author.image,
       array_agg(tag.tag_name)            as tags,
       COALESCE(favorite_count.count, 0)  as favorite_count,
       COALESCE(is_favorite.exist, false) as is_favorite
FROM article
         INNER JOIN users author on article.user_id = author.id
         LEFT JOIN article_tag tag on article.id = tag.article_id
         LEFT JOIN article_favorite favorite on article.id = favorite.article_id
         LEFT JOIN (SELECT article_id, true as exist FROM article_favorite WHERE favorite_user_id = $1) as is_favorite
                    on article.id = is_favorite.article_id
         LEFT JOIN (SELECT article_id, count(*) as count FROM article_favorite GROUP BY article_id) favorite_count
                   on article.id = favorite_count.article_id
WHERE article.deleted = false
  and ($2::text is null or article.id IN (SELECT article_id
                                            FROM article_tag
                                            WHERE tag_name = $2))
  and ($3::text is null or author.user_name = $3)
  and ($4::bigint is null or favorite.favorite_user_id = $4)
GROUP BY article.id, author.id, favorite_count.count, is_favorite.exist
ORDER BY article.id DESC
LIMIT $5 OFFSET $6;
    "#,
        user_id.unwrap_or(0),
        article_query.tag().to_owned(),
        article_query.author().to_owned(),
        favorite_id,
        article_query.limit().unwrap_or(20) as i64,
        article_query.offset().unwrap_or(0) as i64,
    )
        .fetch_all(&app_state.pool)
        .await
        .map_err(|err| {
            error!("Failed get articles Error : {}", err.to_string());
            println!("err {}", err.to_string());
            anyhow!("Failed get articles")
        })?;

    let articles = result
        .into_iter()
        .map(|article_entity| article_entity.to_domain())
        .collect::<Vec<ArticleWithFavorite>>();

    info!("Get succeed default articles count : {}", articles.len());

    Ok(articles)
}

pub async fn get_feed_articles_by_respository(
    user_id: i64,
    limit: i64,
    offset: i64,
    app_state: ArcAppState,
) -> config::Result<Vec<ArticleWithFavorite>> {
    let article_entity_list = sqlx::query_as!(
        MultipleArticleEntity,
        r#"
SELECT article.id                          as article_id,
       article.slug,
       article.title,
       article.description,
       article.body,
       article.created_at,
       article.updated_at,
       author.id                           as user_id,
       author.user_name,
       author.bio,
       author.image,
       array_agg(tag.tag_name)             as tags,
       COALESCE(favorite_count.count, 0)   AS favorite_count,
       COALESCE(is_favorite.exists, false) AS "is_favorite!"
FROM article
         inner JOIN users author on article.user_id = author.id
         LEFT JOIN article_tag tag on article.id = tag.article_id
         LEFT JOIN (SELECT article_id, COUNT(*) as count FROM article_favorite GROUP BY article_id) favorite_count
                   ON article.id = favorite_count.article_id
         LEFT JOIN (SELECT article_id, true as exists
                    FROM article_favorite
                    WHERE favorite_user_id = $1) is_favorite ON article.id = is_favorite.article_id
WHERE article.deleted = false
  and author.id IN (SELECT user_follow.following_id FROM user_follow)
GROUP BY article.id, author.id, created_at, favorite_count.count, is_favorite.exists
ORDER BY article.id DESC
LIMIT $2 OFFSET $3;
        "#,
        user_id,
        limit,
        offset
    )
        .fetch_all(&app_state.pool)
        .await
        .map_err(|err| {
            error!("Failed get feed articles {}", err);
            anyhow!("Failed get feed articles")
        })?;

    let articles = article_entity_list
        .into_iter()
        .map(MultipleArticleEntity::to_domain)
        .collect::<Vec<ArticleWithFavorite>>();

    Ok(articles)
}

#[derive(Debug, FromRow, Encode)]
struct MultipleArticleEntity {
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
    tags: Option<Vec<String>>,
    favorite_count: Option<i64>,
    is_favorite: bool,
}

impl MultipleArticleEntity {
    fn to_domain(self) -> ArticleWithFavorite {
        let author = Author::new(self.user_id, self.user_name, self.bio, self.image);

        let tags = if let Some(tags) = self.tags {
            let tags = tags
                .into_iter()
                .map(|tag| Tag::new(tag))
                .collect::<Vec<Tag>>();
            Some(tags)
        } else {
            None
        };

        let article = Article::new(
            self.article_id,
            self.title,
            self.description,
            self.body,
            self.favorite_count.unwrap(),
            tags,
            author,
        );

        ArticleWithFavorite::new(article, self.is_favorite)
    }
}

#[derive(Debug, FromRow, Encode)]
struct SingleArticleEntity {
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
    tags: Option<Vec<String>>,
    favorite_count: Option<i64>,
}

impl SingleArticleEntity {
    fn to_domain(self) -> Article {
        let author = Author::new(self.user_id, self.user_name, self.bio, self.image);

        let tags = if let Some(tags) = self.tags {
            let tags = tags
                .into_iter()
                .map(|tag| Tag::new(tag))
                .collect::<Vec<Tag>>();
            Some(tags)
        } else {
            None
        };

        Article::new(
            self.article_id,
            self.title,
            self.description,
            self.body,
            self.favorite_count.unwrap(),
            tags,
            author,
        )
    }
}

mod tests {
    use std::sync::Arc;

    use crate::article::application::article_repository::{
        get_default_articles_by_repository, get_single_article_by_repository,
    };
    use crate::article::application::get_articles_default_usecase::ListArticleRequest;
    use crate::config::app_state::init_app_state;

    #[tokio::test]
    async fn get_single_article() {
        let app_state = init_app_state().await;

        let article = get_single_article_by_repository(
            String::from("Hello-mangjoo-2478"),
            Arc::new(app_state),
        )
        .await;

        assert_eq!(article.is_ok(), true);
    }

    #[tokio::test]
    async fn get_list_article() {
        let request =
            ListArticleRequest::new(Some(String::from("mangjoo")), None, None, None, None);

        let result =
            get_default_articles_by_repository(None, request, Arc::new(init_app_state().await))
                .await;

        assert_eq!(result.is_ok(), true);
    }
}
