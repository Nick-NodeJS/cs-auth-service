use std::collections::HashMap;

use anyhow::Result;

use redis::{Client, Cmd, Connection};
use redis::{Commands, FromRedisValue, ToRedisArgs};
use serde::Serialize;

use crate::app::models::session::Session;
use crate::app::services::traits::session_storage::SessionStorage;
use crate::config::redis_config::RedisConfig;

use super::common::CacheServiceType;
use super::error::CacheServiceError;

use mockall::automock;

#[derive(Clone, Debug)]
pub struct RedisCacheService {
    pub client: Client,
}

impl RedisCacheService {
    pub fn new(service_type: CacheServiceType) -> Result<Self, CacheServiceError> {
        let redis_config = RedisConfig::new();
        let database = match service_type {
            CacheServiceType::Session => redis_config.session_database,
            CacheServiceType::Google => redis_config.google_database,
            CacheServiceType::User => redis_config.user_database,
        };
        let client = Client::open(redis_config.get_redis_url(database))?;
        Ok(RedisCacheService { client })
    }

    pub fn get_connection(&self) -> Result<Connection, CacheServiceError> {
        let connection = self.client.get_connection()?;
        Ok(connection)
    }

    pub fn struct_to_cache_string<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string::<T>(data)
    }

    pub fn set(&mut self, key: &str, items: (&str, String)) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        connection.hset(key, items.0, items.1)?;
        Ok(())
    }

    pub fn get_all_set_values(
        &mut self,
        key: &str,
    ) -> Result<HashMap<String, String>, CacheServiceError> {
        let mut connection = self.get_connection()?;
        let result: HashMap<String, String> = connection.hgetall(key)?;
        Ok(result)
    }

    pub fn delete_values(&mut self, keys: Vec<String>) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        let mut cmd = Cmd::new();
        cmd.arg("DEL");
        for key in keys {
            cmd.arg(&key);
        }
        cmd.query(&mut connection)?;
        Ok(())
    }

    pub fn delete_set_values(
        &mut self,
        hset_key: &str,
        keys: Vec<String>,
    ) -> Result<(), CacheServiceError> {
        let mut connection = self.get_connection()?;
        let mut cmd = Cmd::new();
        cmd.arg("HDEL").arg(hset_key);
        for key in keys {
            cmd.arg(&key);
        }
        cmd.query(&mut connection)?;
        Ok(())
    }

    pub fn get_value<T>(&self, key: &str) -> Result<Option<T>, CacheServiceError>
    where
        T: FromRedisValue,
    {
        let mut connection = self.get_connection()?;
        let value: Option<T> = connection.get(key)?;
        Ok(value)
    }

    pub fn get_values<T>(&mut self, keys: Vec<String>) -> Result<Vec<Option<T>>, CacheServiceError>
    where
        T: FromRedisValue,
    {
        let mut connection = self.get_connection()?;
        let values: Vec<Option<T>> = connection.mget(keys)?;
        Ok(values)
    }

    pub fn set_value_with_ttl<T>(
        &mut self,
        key: &str,
        value: T,
        seconds: u64,
    ) -> Result<(), CacheServiceError>
    where
        T: ToRedisArgs,
    {
        let mut connection = self.get_connection()?;
        let _: () = connection.set_ex(key, value, seconds)?;
        Ok(())
    }
}

impl SessionStorage for RedisCacheService {
    fn load(&self, key: &str) -> Result<Option<Session>, CacheServiceError> {
        self.get_value::<Session>(&Session::get_session_key(key).as_ref())
    }
}
