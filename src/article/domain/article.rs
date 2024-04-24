use chrono::{DateTime, Utc};

use crate::article::domain::tag::Tag;
use crate::user::domain::user::User;

#[derive(Debug)]
pub struct Article {
    id: i64,
    slug: String,
    title: String,
    description: String,
    tag_list: Option<Vec<Tag>>,
    body: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author: User,
}

impl Article {
    pub fn new(
        id: i64,
        title: String,
        description: String,
        body: String,
        tag_list: Option<Vec<Tag>>,
        author: User,
    ) -> Self {
        Article {
            id,
            slug: title.replace(" ", "-"),
            title,
            description,
            tag_list,
            body,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            author,
        }
    }


    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn slug(&self) -> &String {
        &self.slug
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn tag_list(&self) -> &Option<Vec<Tag>> {
        &self.tag_list
    }
    pub fn body(&self) -> &String {
        &self.body
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn author(&self) -> &User {
        &self.author
    }
}