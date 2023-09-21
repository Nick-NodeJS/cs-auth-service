use redis::Client;
use redis::AsyncCommands;

use crate::config::redis_config::RedisConfig;

pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub fn new() -> Self {
        let redis_config = RedisConfig::new();
        let client = Client::open(redis_config.get_redis_url())
            .expect("Failed to create Redis client");
        RedisService { client }
    }

    // For future use
    // pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
    //     let mut connection = self.client.get_async_connection().await?;
    //     connection.set(key, value).await?;
    //     Ok(())
    // }

    pub async fn set_value_with_ttl(&self, key: &str, value: &str, milliseconds: usize) -> Result<(), redis::RedisError> {
        let mut connection = self.client.get_async_connection().await?;
        connection.pset_ex(key, value, milliseconds).await?;
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut connection = self.client.get_async_connection().await?;
        let value: Option<String> = connection.get(key).await?;
        Ok(value)
    }
}