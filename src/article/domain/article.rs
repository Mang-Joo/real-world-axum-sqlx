use chrono::{DateTime, Utc};

use crate::article::domain::tag::Tag;

#[derive(Debug, Clone)]
pub struct Article {
    id: i64,
    slug: String,
    title: String,
    description: String,
    tag_list: Option<Vec<Tag>>,
    body: String,
    favorite_count: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author: Author,
}

#[derive(Debug, Clone)]
pub struct Author {
    id: i64,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
}

impl Article {
    pub fn new(
        id: i64,
        title: String,
        description: String,
        body: String,
        favorite_count: i64,
        tag_list: Option<Vec<Tag>>,
        author: Author,
    ) -> Self {
        Article {
            id,
            slug: title.replace(" ", "-"),
            title,
            description,
            tag_list,
            favorite_count,
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
    pub fn author(&self) -> &Author {
        &self.author
    }
    pub fn favorite_count(&self) -> i64 {
        self.favorite_count
    }
}

impl Author {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn image(&self) -> &Option<String> {
        &self.image
    }
    pub fn new(
        id: i64,
        user_name: String,
        bio: Option<String>,
        image: Option<String>,
    ) -> Self {
        Self { id, user_name, bio, image }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleWithFavorite {
    article: Article,
    is_favorite: bool,
}

impl ArticleWithFavorite {
    pub fn new(article: Article, is_favorite: bool) -> Self {
        Self { article, is_favorite }
    }


    pub fn article(&self) -> &Article {
        &self.article
    }
    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }
}