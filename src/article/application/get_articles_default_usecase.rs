use crate::article::domain::article::ArticleWithFavorite;
use crate::config;

pub async fn get_article_default(
    user_id: Option<i64>,
    request: ListArticleRequest,
) -> config::Result<Vec<ArticleWithFavorite>> {
    todo!()
}


#[derive(Debug, Clone)]
pub struct ListArticleRequest {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ListArticleRequest {

    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }
    pub fn author(&self) -> &Option<String> {
        &self.author
    }
    pub fn favorited(&self) -> &Option<String> {
        &self.favorited
    }
    pub fn limit(&self) -> Option<i64> {
        self.limit
    }
    pub fn offset(&self) -> Option<i64> {
        self.offset
    }
    pub fn new(tag: Option<String>, author: Option<String>, favorited: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Self {
        Self { tag, author, favorited, limit, offset }
    }
}