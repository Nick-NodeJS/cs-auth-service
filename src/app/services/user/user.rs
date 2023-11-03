use crate::app::app_error::AppError;

pub struct GoogleProfile {
  user_id: String,
  name: Option<String>,
  email: Option<String>,
}

pub struct FacebookProfile {
  user_id: String,
  name: Option<String>,
  email: Option<String>,
}

pub enum UserProfile {
  GoogleProfile,
  FacebookProfile,
}

pub enum UserActiveProfile {
  Google,
  Facebook,
}

pub struct User {
  id: String,
  active_profile: UserActiveProfile,
  google: Option<UserProfile>,
  facebook: Option<UserProfile>,
}

impl User {
  pub fn new(id: String, active_profile: UserActiveProfile, profile: UserProfile ) -> Result<User, AppError> {
    let mut user = User {
      id,
      active_profile,
      google: None,
      facebook: None,
    };
    match active_profile {
        UserActiveProfile::Google => user.google = Some(profile),
        UserActiveProfile::Facebook => user.facebook = Some(profile),
    }
    Ok(user)
  }
}