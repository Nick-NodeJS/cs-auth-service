use redis::Client;
use redis::AsyncCommands;

pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url)
            .expect("Failed to create Redis client");
        RedisService { client }
    }

    pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let mut connection = self.client.get_async_connection().await?;
        connection.set(key, value).await?;
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut connection = self.client.get_async_connection().await?;
        let value: Option<String> = connection.get(key).await?;
        Ok(value)
    }
}