use std::collections::HashMap;

use anyhow::Result;

use redis::{Client, Connection, FromRedisValue, RedisResult, ToRedisArgs};
use redis::{Commands, RedisError};
use serde_json::Value;

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
    client: Client,
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

    pub fn hmset(&mut self, key: &str, items: &[(&str, String)]) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        connection.hset_multiple(key, items)?;
        Ok(())
    }

    pub fn hmget(&mut self, key: &str) -> Result<HashMap<String, String>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let result: HashMap<String, String> = connection.hgetall(key)?;
        Ok(result)
    }

    pub fn set_value_with_ttl(
        &mut self,
        key: &str,
        value: &str,
        seconds: usize,
    ) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        connection.set_ex(key, value, seconds)?;
        Ok(())
    }

    pub fn get_value(&mut self, key: &str) -> Result<Option<String>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let value: Option<String> = connection.get(key)?;
        Ok(value)
    }

    fn get_connection(&self) -> Result<Connection, CacheServiceError> {
        let connection = self.client.get_connection()?;
        Ok(connection)
    }
}

// TODO: investigate how to implement Debug for Redis Service
// impl fmt::Debug for RedisService {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("RedisService")
//             .field("Client ", &self.client)  // Replace with the actual field
//             .finish()
//     }
// }
