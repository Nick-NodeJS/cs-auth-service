use std::collections::HashMap;

use actix_web::web;
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};
use serde::{Deserialize, Serialize};

use crate::app::{models::session::Session, services::cache::service::RedisCacheService};

use super::error::ProviderError;

pub struct CallbackQueryData {
    pub code: String,
    pub state: String,
}

/// get code and state params from query string
pub fn parse_callback_query_string(query_string: &str) -> Result<CallbackQueryData, ProviderError> {
    let try_params = web::Query::<HashMap<String, String>>::from_query(query_string);
    match try_params {
        Ok(params) => {
            let code: String;
            if let Some(code_string) = params.get("code") {
                code = code_string.to_owned();
            } else {
                return Err(ProviderError::CodeParamError);
            };
            let state: String;
            if let Some(state_string) = params.get("state") {
                state = state_string.to_owned();
            } else {
                return Err(ProviderError::StateParamError);
            };
            return Ok(CallbackQueryData { code, state });
        }
        Err(err) => {
            log::error!("{}", err.to_string());
            return Err(ProviderError::QueryStringError);
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginCacheData {
    pub pkce_code_verifier: String,
    pub session: Session,
}

impl FromRedisValue for LoginCacheData {
    fn from_redis_value(value: &RedisValue) -> RedisResult<LoginCacheData> {
        match *value {
            RedisValue::Data(ref data) => Ok(serde_json::from_slice::<LoginCacheData>(data)?),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!("(response was {:?})", value),
            ))),
        }
    }
}

pub fn get_login_cache_data_by_state(
    cache_service: &RedisCacheService,
    state: &str,
) -> Result<LoginCacheData, ProviderError> {
    // process code and state
    let login_cache_data_value =
        cache_service.get_value::<LoginCacheData>(state.clone().as_ref())?;
    if let Some(login_cache_data) = login_cache_data_value {
        Ok(login_cache_data)
    } else {
        log::debug!("No callback request state {} in Redis", state);
        return Err(ProviderError::CallbackStateCacheError);
    }
}
