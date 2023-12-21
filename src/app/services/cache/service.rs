use std::collections::HashMap;

use anyhow::Result;

use chrono::{DateTime, Utc};
use redis::{Client, Connection, FromRedisValue};
use redis::{Commands, ToRedisArgs};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use crate::app::models::session::Session;
use crate::app::services::traits::session_storage::SessionStorage;
use crate::config::redis_config::RedisConfig;

use super::error::CacheServiceError;

#[derive(Debug)]
pub enum CacheServiceType {
    Google,
    Session,
    User,
}
#[derive(Clone, Debug)]
pub struct CacheService {
    pub client: Client,
}

impl CacheService {
    pub fn new(service_type: CacheServiceType) -> Result<Self, CacheServiceError> {
        let redis_config = RedisConfig::new();
        let database = match service_type {
            CacheServiceType::Session => redis_config.session_database,
            CacheServiceType::Google => redis_config.google_database,
            CacheServiceType::User => redis_config.user_database,
        };
        let client = Client::open(redis_config.get_redis_url(database))?;
        Ok(CacheService { client })
    }

    pub fn hset(&mut self, key: &str, items: (&str, String)) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        connection.hset(key, items.0, items.1)?;
        Ok(())
    }

    pub fn hgetall(&mut self, key: &str) -> Result<HashMap<String, String>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let result: HashMap<String, String> = connection.hgetall(key)?;
        Ok(result)
    }

    pub fn mget<T: FromRedisValue>(
        &mut self,
        keys: Vec<String>,
    ) -> Result<Vec<Option<T>>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let result: Vec<Option<T>> = connection.mget(keys)?;
        Ok(result)
    }

    pub fn get_value<T: FromRedisValue>(&self, key: &str) -> Result<Option<T>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let value: Option<T> = connection.get(key)?;
        Ok(value)
    }

    pub fn set_value_with_ttl<T>(
        &mut self,
        key: &str,
        value: T,
        seconds: usize,
    ) -> Result<(), CacheServiceError>
    where
        T: ToRedisArgs,
    {
        let mut connection = self.get_connection()?;
        connection.set_ex(key, value, seconds)?;
        Ok(())
    }

    pub fn struct_to_cache_string<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string::<T>(data)
    }

    fn get_connection(&self) -> Result<Connection, CacheServiceError> {
        let connection = self.client.get_connection()?;
        Ok(connection)
    }
}

impl SessionStorage for CacheService {
    fn load(&self, key: &str) -> Result<Option<Session>, CacheServiceError> {
        self.get_value::<Session>(&Session::get_session_key(key).as_ref())
    }
}
