use serde::Serialize;

use crate::profile::domain::model::Profile;

#[derive(Serialize)]
pub struct ProfileResponseDto<T: Serialize> {
    pub profile: T,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl From<Profile> for ProfileResponse {
    fn from(profile: Profile) -> Self {
        Self {
            username: profile.username().to_string(),
            bio: profile.bio().cloned(),
            image: profile.image().cloned(),
            following: profile.following(),
        }
    }
}
