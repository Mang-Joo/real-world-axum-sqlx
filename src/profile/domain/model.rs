pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl Profile {
    pub fn new(
        username: String,
        bio: Option<String>,
        image: Option<String>,
        following: bool,
    ) -> Self {
        Self {
            username,
            bio,
            image,
            following,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn bio(&self) -> Option<&String> {
        self.bio.as_ref()
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn following(&self) -> bool {
        self.following
    }
}
