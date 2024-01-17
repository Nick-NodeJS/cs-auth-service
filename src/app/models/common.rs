use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AuthProviders {
    CyberSherlock,
    Google,
    Facebook,
}
impl AuthProviders {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn is_equal(&self, provider: &AuthProviders) -> bool {
        self.to_string() == provider.to_string()
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

pub mod datetime_as_mongo_bson {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(val: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = mongodb::bson::DateTime::from(val.to_owned());
        datetime.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        value
            .parse()
            .map_err(|_| serde::de::Error::custom("Error parsing DateTime from string"))
    }
}
