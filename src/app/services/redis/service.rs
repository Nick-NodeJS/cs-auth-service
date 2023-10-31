use std::ops::DerefMut;
use std::time::Duration;

use anyhow::Result;
use r2d2::Pool;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis;

use crate::app::app_error::AppError;
use crate::config::redis_config::RedisConfig;

pub struct RedisService {
    pool: Pool<RedisConnectionManager>,
}

impl RedisService {
    pub fn new() -> Result<Self> {
        let redis_config = RedisConfig::new();
        let manager = RedisConnectionManager::new(redis_config.get_redis_url())?;
        // let pool = Pool::new(manager)?;
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(5))
            .build(manager)?;
        Ok(RedisService { pool })
    }

    // For future use
    // pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
    //     let mut connection = self.client.get_async_connection().await?;
    //     connection.set(key, value).await?;
    //     Ok(())
    // }

    pub fn set_value_with_ttl(&mut self, key: &str, value: &str, milliseconds: usize) -> Result<(), AppError> {
        let mut connection = self.get_connection()?;
        redis::cmd("SET").arg(key).arg(value).arg("EX").arg(milliseconds).query(connection.deref_mut())?;
        Ok(())
    }

    pub fn get_value(&mut self, key: &str) -> Result<Option<String>, AppError> {
        let mut connection = self.get_connection()?;
        let value: Option<String> = redis::cmd("GET").arg(key).query(connection.deref_mut())?;
        Ok(value)
    }

    fn get_connection(&self) -> Result<PooledConnection<RedisConnectionManager>, AppError> {
        let connection = self.pool.get()?;
        Ok(connection)
    }
}