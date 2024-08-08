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
