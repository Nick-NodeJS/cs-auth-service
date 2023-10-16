use redis::{Connection, Commands, RedisError};

pub fn get_connection(redis_url: &str) -> Result<Connection, RedisError> {
  let client = redis::Client::open(redis_url)?;
  let connection = client.get_connection()?;
  Ok(connection)
}

// For future use
// pub async fn set_value(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
//     let mut connection = self.client.get_async_connection().await?;
//     connection.set(key, value).await?;
//     Ok(())
// }

pub async fn set_value_with_ttl(connection: &mut Connection, key: &str, value: &str, milliseconds: usize) -> Result<(), RedisError> {
  //let mut connection  = self.client.get_connection()?;
  let _:() = connection.set_ex(key, value, milliseconds)?;
  //redis::cmd("SET").arg(key).arg(value).arg("EX").arg(milliseconds).query(connection)?;
  Ok(())
}

pub async fn get_value(connection: &mut Connection, key: &str) -> Result<Option<String>, RedisError> {
  //let mut connection = self.client.get_connection()?;
  let value: Option<String> = connection.get(key)?;//redis::cmd("GET").arg(key).query(connection)?;
  Ok(value)
}