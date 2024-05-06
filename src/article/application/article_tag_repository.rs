use anyhow::anyhow;
use log::error;
use sqlx::{Postgres, QueryBuilder, Transaction};

use crate::article::domain::tag::Tag;
use crate::config;

pub async fn save_article_and_tags(
    article_id: i64,
    tags: &Vec<Tag>,
    db_pool: &mut Transaction<'_, Postgres>,
) -> config::Result<()> {
    QueryBuilder::new("INSERT INTO article_tag (article_id, tag_name)")
        .push_values(tags, |mut builder, tag| {
            builder.push_bind(article_id).push_bind(tag.tag());
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

struct TagEntity {
    tag_name: String,
}

impl TagEntity {
    fn to_domain(self) -> Tag {
        Tag::new(self.tag_name)
    }
}