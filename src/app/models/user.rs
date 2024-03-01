use bson::doc;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use redis::{
    ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs,
    Value as RedisValue,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::app::services::storage::service::CollectionType;

use super::{
    common::{datetime_as_mongo_bson, AuthProviders},
    user_profile::{CyberSherlockProfile, FacebookProfile, GoogleProfile, UserProfile},
};
pub type UserId = ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: UserId,
    pub active_profile: AuthProviders,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cyber_sherlock: Option<CyberSherlockProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google: Option<GoogleProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook: Option<FacebookProfile>,
    #[serde(with = "datetime_as_mongo_bson")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "datetime_as_mongo_bson")]
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(profile: UserProfile) -> User {
        let now = Utc::now();
        let mut user = User {
            id: User::generate_user_id(),
            active_profile: AuthProviders::CyberSherlock,
            cyber_sherlock: None,
            google: None,
            facebook: None,
            created_at: now,
            updated_at: now,
        };
        match profile {
            UserProfile::CyberSherlock(cyber_sherlock_profile) => {
                // CyberSherlock profile has system user id
                user.id = cyber_sherlock_profile.user_id.clone();
                user.cyber_sherlock = Some(cyber_sherlock_profile);
            }
            UserProfile::Google(google_profile) => {
                user.google = Some(google_profile);
                user.active_profile = AuthProviders::Google;
            }
            UserProfile::Facebook(facebook_profile) => {
                user.facebook = Some(facebook_profile);
                user.active_profile = AuthProviders::Facebook;
            }
        }
        user
    }

    pub fn to_json_with_hash(&self) -> Value {
        json!({
            // Need to keep underscore `_id` because of MongoDB usage
            "_id": self.id,
            "active_profile": self.active_profile,
            "cyber_sherlock": self.cyber_sherlock,
            "google": self.google,
            "facebook": self.facebook,
            "created_at": self.created_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "updated_at": self.updated_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        })
    }
    pub fn to_json(&self) -> Value {
        let cyber_sherlock = match &self.cyber_sherlock {
            None => Value::Null,
            Some(cyber_sherlock_profile) => cyber_sherlock_profile.to_json(),
        };
        let mut json_user = self.to_json_with_hash();
        if self.cyber_sherlock.is_some() {
            *json_user.get_mut("cyber_sherlock").unwrap() = cyber_sherlock;
        }
        json_user
    }

    pub fn get_user_cache_key(user_id: &str) -> String {
        format!("user::{}", user_id)
    }

    pub fn generate_user_id() -> UserId {
        ObjectId::new()
    }
}

impl CollectionType for User {}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(self.to_json_with_hash().to_string().as_bytes());
    }
}

impl FromRedisValue for User {
    fn from_redis_value(value: &RedisValue) -> RedisResult<User> {
        match *value {
            RedisValue::Data(ref data) => {
                let user = serde_json::from_slice::<User>(data)?;
                Ok(user)
            }
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", value),
            ))),
        }
    }
}
