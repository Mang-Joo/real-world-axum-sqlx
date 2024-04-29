use std::sync::Arc;
use crate::article::domain::article::Article;
use crate::config;
use crate::config::app_state::AppState;

pub async fn single_article(
    slug: String,
    app_state: Arc<AppState>
) -> config::Result<Article> {


    todo!()
}