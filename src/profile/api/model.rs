use serde::{Deserialize, Serialize};

use crate::profile::domain::profile::Profile;

#[derive(Serialize)]
pub struct ProfileResponseDto<T> {
    pub profile: T,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl ProfileResponse {
    pub fn new(profile: Profile) -> Self {
        Self {
            username: profile.username().to_string(),
            bio: profile.bio().to_owned().cloned(),
            image: profile.image().to_owned().cloned(),
            following: profile.following(),
        }
    }
}
