use std::sync::Arc;

use crate::user::domain::hash_password::HashPassword;

#[derive(Debug)]
pub struct User {
    id: u32,
    pub email: String,
    password: Arc<String>,
    pub user_name: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

unsafe impl Send for User {}

unsafe impl Sync for User {}

impl User {
    pub fn new(
        id: u32,
        email: String,
        password: String,
        user_name: String,
        bio: Option<String>,
        image: Option<String>,
    ) -> User {
        User {
            id,
            email,
            password: Arc::new(password),
            user_name,
            bio,
            image,
        }
    }

    pub async fn not_verify_password(&self, input_password: String, hash: &(dyn HashPassword + Send + Sync)) -> bool {
        !hash.verify(input_password, &self.password)
            .await
    }
}