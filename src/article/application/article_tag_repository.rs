use anyhow::anyhow;
use log::error;
use sqlx::{MySql, QueryBuilder, Transaction};

use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::db::DbPool;

pub async fn save_article_and_tags(
    article_id: i64,
    tags: &Vec<Tag>,
    db_pool: &mut Transaction<'_, MySql>,
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