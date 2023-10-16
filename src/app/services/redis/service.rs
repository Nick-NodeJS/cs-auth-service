use anyhow::Result;
use redis::{Connection, ConnectionLike, Client};
//use redis::AsyncCommands;
use redis::Commands;

use crate::config::redis_config::RedisConfig;

pub struct RedisService {
    connection: Connection,
}

impl RedisService {
    pub fn new() -> Result<Self> {
        let redis_config = RedisConfig::new();
        let client = redis::Client::open(redis_config.get_redis_url())?;
        let connection = client.get_connection()?;
        Ok(RedisService { connection })
    }

    // For future use
    // pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
    //     let mut connection = self.client.get_async_connection().await?;
    //     connection.set(key, value).await?;
    //     Ok(())
    // }

    pub async fn set_value_with_ttl(&mut self, key: &str, value: &str, milliseconds: usize) -> Result<(), redis::RedisError> {
        //let mut connection  = self.client.get_connection()?;
        let _:() = self.connection.set_ex(key, value, milliseconds)?;
        //redis::cmd("SET").arg(key).arg(value).arg("EX").arg(milliseconds).query(connection)?;
        Ok(())
    }

    pub async fn get_value(&mut self, key: &str) -> Result<Option<String>, redis::RedisError> {
        //let mut connection = self.client.get_connection()?;
        let value: Option<String> = self.connection.get(key)?;//redis::cmd("GET").arg(key).query(connection)?;
        Ok(value)
    }
}