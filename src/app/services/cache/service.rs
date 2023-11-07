use std::fmt;
use std::ops::DerefMut;
use std::time::Duration;

use anyhow::Result;
// use r2d2::Pool;
// use r2d2::PooledConnection;
// use r2d2_redis::RedisConnectionManager;
// use r2d2_redis::redis;

use redis::{ Client, Connection };
use redis::Commands;

use crate::app::app_error::AppError;
use crate::config::redis_config::RedisConfig;

#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub fn new() -> Result<Self> {
        let redis_config = RedisConfig::new();
        // let initial_nodes = vec![redis_config.get_redis_url()];
        let client = Client::open(redis_config.get_redis_url())?;
        // let manager = RedisConnectionManager::new(redis_config.get_redis_url())?;
        // let pool = Pool::new(manager)?;
        // let pool = Pool::builder()
        //     .connection_timeout(Duration::from_secs(5))
        //     .build(manager)?;
        Ok(CacheService { client })
    }

    // For future use
    // pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
    //     let mut connection = self.client.get_async_connection().await?;
    //     connection.set(key, value).await?;
    //     Ok(())
    // }

    pub fn set_value_with_ttl(&mut self, key: &str, value: &str, milliseconds: usize) -> Result<(), AppError> {
        let mut connection = self.get_connection()?;
        connection.set_ex(key, value, milliseconds)?;
        // redis::cmd("SET").arg(key).arg(value).arg("EX").arg(milliseconds).query(connection.deref_mut())?;
        Ok(())
    }

    pub fn get_value(&mut self, key: &str) -> Result<Option<String>, AppError> {
        let mut connection = self.get_connection()?;
        let value: Option<String> = connection.get(key)?;//redis::cmd("GET").arg(key).query(connection.deref_mut())?;
        Ok(value)
    }

    fn get_connection(&self) -> Result<Connection, AppError> {
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