use std::sync::Arc;

use crate::user::domain::hash_password::HashPassword;

#[derive(Debug)]
pub struct User {
    id: u64,
    email: String,
    password: Arc<String>,
    user_name: String,
    bio: Arc<Option<String>>,
    image: Arc<Option<String>>,
}

unsafe impl Send for User {}

unsafe impl Sync for User {}

impl User {
    pub fn new(
        id: u64,
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
            bio: Arc::new(bio),
            image: Arc::new(image),
        }
    }

    pub async fn not_verify_password(&self, input_password: String, hash: &(dyn HashPassword + Send + Sync)) -> bool {
        !hash.verify(input_password, &self.password)
            .await
    }


    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password(&self) -> &String {
        &self.password.as_ref()
    }
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio.as_ref()
    }
    pub fn image(&self) -> &Option<String> {
        &self.image.as_ref()
    }
}