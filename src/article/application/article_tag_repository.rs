use anyhow::anyhow;
use log::error;
use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::db::DbPool;

pub async fn save_article_and_tags(
    article_id: i64,
    tags: &Vec<Tag>,
    db_pool: &mut Transaction<'_, Postgres>,
) -> config::Result<()> {
    QueryBuilder::new("INSERT INTO article_tag (article_id, tag_name)")
        .push_values(
            tags,
            |mut builder, tag| {
                builder
                    .push_bind(article_id)
                    .push_bind(tag.tag());
            })
        .build()
        .fetch_all(&mut **db_pool)
        .await
        .map_err(|err| {
            error!("Failed save article_tag {}", err);
            anyhow!("Failed save article_tag")
        })?
        .clear();

    Ok(())
}

pub async fn get_tags_by_article_id(
    article_id: i64,
    db_pool: &DbPool,
) -> config::Result<Option<Vec<Tag>>> {
    let result = sqlx::query_as!(
        TagEntity,
        "SELECT tag_name
        FROM article_tag
        WHERE article_id = $1",
        article_id
    ).fetch_all(db_pool)
        .await;

    match result {
        Ok(response) => {
            let tags = response.into_iter()
                .map(|entity| entity.to_domain())
                .collect::<Vec<Tag>>();
            Ok(Some(tags))
        }
        Err(err) => {
            return Err(anyhow!("Failed get tags {}", err.to_string()));
        }
    }
}

struct TagEntity {
    tag_name: String,
}

impl TagEntity {
    fn to_domain(self) -> Tag {
        Tag::new(self.tag_name)
    }
}

mod tests {
    use crate::article::application::article_tag_repository::get_tags_by_article_id;
    use crate::config::db::init_db;

    #[tokio::test]
    async fn get_tags_test() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world")).await;

        let result = get_tags_by_article_id(0, &db).await;

        let tags = result.unwrap()
            .unwrap();

        assert_eq!(tags.len(), 0);
    }
}