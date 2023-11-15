use serde::{Deserialize, Serialize};

use super::error::UserServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FacebookProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserProfile {
    Google(GoogleProfile),
    Facebook(FacebookProfile),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserActiveProfile {
    Google,
    Facebook,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    active_profile: UserActiveProfile,
    google: Option<GoogleProfile>,
    facebook: Option<FacebookProfile>,
}

impl User {
    pub fn new(
        id: String,
        active_profile: UserActiveProfile,
        profile: UserProfile,
    ) -> Result<User, UserServiceError> {
        let mut user = User {
            id,
            active_profile,
            google: None,
            facebook: None,
        };
        match profile {
            UserProfile::Google(google_profile) => user.google = Some(google_profile),
            UserProfile::Facebook(facebook_profile) => user.facebook = Some(facebook_profile),
        }
        Ok(user)
    }
}
