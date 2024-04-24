use anyhow::anyhow;
use log::error;
use sqlx::{QueryBuilder};

use crate::article::domain::tag::Tag;
use crate::config;
use crate::config::db::DbPool;

pub async fn save_tags(
    tags: &Vec<Tag>,
    db_pool: &DbPool,
) -> config::Result<Vec<Tag>> {
    QueryBuilder::new("INSERT IGNORE INTO tag (tag_name)")
        .push_values(tags, |mut builder, tag| {
            builder.push_bind(tag.tag().to_owned());
        })
        .build()
        .fetch_all(db_pool)
        .await
        .map_err(|err| {
            error!("Failed save tag {}", err);
            eprintln!("err: {err}");
            anyhow!("Failed save tags")
        })?
        .clear();

    Ok(tags.to_owned())
}

struct TagEntity {
    id: i64,
    tag_name: String,
}

impl TagEntity {
    fn from_domain(tag: Tag) -> Self {
        TagEntity {
            id: 0,
            tag_name: tag.tag().to_owned(),
        }
    }
}


mod tests {
    use crate::article::application::tag_repository::save_tags;
    use crate::article::domain::tag::Tag;
    use crate::config::db::init_db;

    #[tokio::test]
    async fn save_tags_test() {
        let db = init_db(String::from("mysql://root:akdwn1212!@146.56.115.136:3306/real_world"))
            .await;

        let tags = vec![Tag::new(String::from("Helloo")), Tag::new(String::from("Hio")), Tag::new(String::from("No"))];

        let saved_tags = save_tags(&tags, &db).await
            .unwrap();

        assert_eq!(saved_tags.len(), 3);
    }
}