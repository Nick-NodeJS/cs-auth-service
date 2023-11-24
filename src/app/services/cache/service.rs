use anyhow::Result;

use redis::{Client, Connection};
use redis::{Commands, RedisError};

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

    // For future use
    // pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
    //     let mut connection = self.client.get_async_connection().await?;
    //     connection.set(key, value).await?;
    //     Ok(())
    // }

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
