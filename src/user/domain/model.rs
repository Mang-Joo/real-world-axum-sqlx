use crate::{auth::hash_password::DynHashPassword, config::RealWorldResult};

pub struct UserRegistry {
    user_name: String,
    email: String,
    password: String,
}

impl UserRegistry {
    pub fn new(user_name: String, email: String, password: String) -> Self {
        Self {
            user_name,
            email,
            password,
        }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn hash_of_password(
        self,
        hash_password: &DynHashPassword,
    ) -> RealWorldResult<UserRegistry> {
        let hashed_password = hash_password.hash(&self.password)?;
        let user_registry = Self {
            password: hashed_password,
            ..self
        };
        Ok(user_registry)
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

pub struct UserLogin {
    email: String,
    password: String,
}

impl UserLogin {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(self) -> String {
        self.password
    }
}

pub struct UserUpdate {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    image: Option<String>,
    bio: Option<String>,
}

impl UserUpdate {
    pub fn new(
        email: Option<String>,
        username: Option<String>,
        password: Option<String>,
        image: Option<String>,
        bio: Option<String>,
    ) -> Self {
        Self {
            email,
            username,
            password,
            image,
            bio,
        }
    }

    pub fn update_non_option_fields(
        self,
        email: String,
        username: String,
        password: String,
        image: Option<String>,
        bio: Option<String>,
    ) -> Self {
        Self {
            email: Some(email),
            username: Some(username),
            password: Some(password),
            image,
            bio,
        }
    }

    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn bio(&self) -> Option<&String> {
        self.bio.as_ref()
    }
}
