#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    email: String,
    password: String,
    user_name: String,
    bio: Option<String>,
    image: Option<String>,
}

impl User {
    pub fn new(
        id: i64,
        email: String,
        password: String,
        user_name: String,
        bio: Option<String>,
        image: Option<String>,
    ) -> User {
        User {
            id,
            email,
            password,
            user_name,
            bio,
            image,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn email(&self) -> &String {
        &self.email
    }
    pub fn password(&self) -> &String {
        &self.password
    }
    pub fn user_name(&self) -> &String {
        &self.user_name
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn image(&self) -> &Option<String> {
        &self.image
    }

    pub fn set_email(self, email: String) -> User {
        User { email, ..self }
    }
    pub fn set_password(self, password: String) -> User {
        User { password, ..self }
    }
    pub fn set_user_name(self, user_name: String) -> User {
        User { user_name, ..self }
    }
    pub fn set_bio(self, bio: Option<String>) -> User {
        User { bio, ..self }
    }
    pub fn set_image(self, image: Option<String>) -> Self {
        User { image, ..self }
    }
}

pub struct AuthUser {
    username: String,
    token: String,
    email: String,
    bio: Option<String>,
    image: Option<String>,
}

impl AuthUser {
    pub fn new(user: User, token: String) -> Self {
        Self {
            username: user.user_name,
            token,
            email: user.email,
            bio: user.bio,
            image: user.image,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.as_ref().map(|s| s.to_owned())
    }

    pub fn image(&self) -> Option<String> {
        self.image.as_ref().map(|s| s.to_owned())
    }
}
