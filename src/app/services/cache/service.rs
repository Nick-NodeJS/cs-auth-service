use std::collections::HashMap;

use anyhow::Result;

use redis::{Client, Connection, FromRedisValue, RedisResult, ToRedisArgs};
use redis::{Commands, RedisError};
use serde_json::Value;

use crate::config::redis_config::RedisConfig;

#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub fn new() -> Result<Self> {
        let redis_config = RedisConfig::new();
        let client = Client::open(redis_config.get_redis_url())?;
        Ok(CacheService { client })
    }

    pub fn hmset(&mut self, key: &str, items: &[(&str, String)]) -> Result<(), RedisError> {
        let mut connection = self.get_connection()?;
        connection.hset_multiple(key, items)?;
        Ok(())
    }

    pub fn hmget(&mut self, key: &str) -> Result<HashMap<String, String>, RedisError> {
        let mut connection = self.get_connection()?;
        let result: HashMap<String, String> = connection.hgetall(key)?;
        Ok(result)
    }

    pub fn set_value_with_ttl(
        &mut self,
        key: &str,
        value: &str,
        seconds: usize,
    ) -> Result<(), RedisError> {
        let mut connection = self.get_connection()?;
        connection.set_ex(key, value, seconds)?;
        Ok(())
    }

    pub fn get_value(&mut self, key: &str) -> Result<Option<String>, RedisError> {
        let mut connection = self.get_connection()?;
        let value: Option<String> = connection.get(key)?;
        Ok(value)
    }

    fn get_connection(&self) -> Result<Connection, RedisError> {
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
