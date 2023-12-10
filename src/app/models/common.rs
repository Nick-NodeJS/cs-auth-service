use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthProviders {
    CyberSherlock,
    Google,
    Facebook,
}
impl AuthProviders {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for AuthProviders {
    type Err = &'static str;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(AuthProviders::Google),
            "facebook" => Ok(AuthProviders::Facebook),
            _ => Err("Invalid AuthProvider"),
        }
    }
}
