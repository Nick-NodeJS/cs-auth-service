use serde_json::{Map, Value};
use std::{collections::HashMap, sync::MutexGuard};

use actix_web::{HttpRequest, HttpResponse, web};

use crate::app::{
  app_data::AppData,
  services::redis::service::RedisService,
};

pub async fn auth_callback(req: HttpRequest, app_data: web::Data<AppData>) -> HttpResponse {
  let redis_service = &app_data.redis_service.lock().unwrap();
  match parse_query_string(req.query_string()) {
      Ok((code, state)) => {
        // process code + state
        if let Some(pkce_code_verifier) = get_state_from_cache(state.clone(), redis_service).await {
          let google_service = app_data.google_service.lock().unwrap();
          match google_service.get_tokens(code, pkce_code_verifier) {
            Ok(tokens) => {
              if let Err(err) = set_user_to_storage(tokens.clone()).await {
                return google_bad_request_error(
                  format!("Error to set tokens to storage: {}",err)
                )
              }
              return return_tokens_as_json(tokens);
            },
            Err(err) => {
              return google_bad_request_error(err);
            },
          }
        } else {
          return google_bad_request_error(
            format!("No callback request state {} in Redis", state)
          )
        }
      },
      Err(error_msg) => {
        return google_bad_request_error(
          format!("Error to parse Google callback request query string: {}", error_msg)
        )
      },
  }
}

async fn set_user_to_storage(tokens: (String, String)) -> Result<(), String> {
  /* TODO:
   - update google service to get OAuth2 cert on initial step(method new)
   - decode access_token -> token data(google service)
   - create database service
   - create new user or update existing user in database
   - set or update cache with token data
   */
  return Ok(())
}

fn return_tokens_as_json(tokens: (String, String)) -> HttpResponse {
  let (access_token, refresh_token) = tokens;
  let mut payload = Map::new();
  payload.insert("access_token".to_string(), Value::String(access_token));
  payload.insert("refresh_token".to_string(), Value::String(refresh_token));
  return HttpResponse::Ok().json(payload);
}
async fn get_state_from_cache(
  code: String,
  redis_service: &MutexGuard<'_, RedisService>,
) -> Option<String> {
    match redis_service.get_value(&code).await {
      Ok(state) => {
        return state;
      },
      Err(err) => {
        log::error!("REDIS SERVICE ERROR: {}", err);
        return None;
      }
  }
}

fn parse_query_string(query_string: &str) -> Result<(String, String), &str> {
  let try_params = web::Query::<HashMap<String, String>>::from_query(
    query_string,
  );
  match try_params {
    Ok(params) => {
      let code: String;
      if let Some(code_string) = params.get("code") {
        code = code_string.to_owned();
      } else {
        return Err("Invalid code parameter")
      };
      let state: String;
      if let Some(state_string) = params.get("state") {
        state = state_string.to_owned();
      } else {
        return Err("Invalid code parameter")
      };
      return Ok((code, state));
    },
    Err(err) => {
      log::error!("{}", err.to_string());
      return Err("Invalid query string")
    },
  }
}

fn google_bad_request_error(err: String) -> HttpResponse {
  log::error!("Bad Google request: {}", err);
  return HttpResponse::BadRequest().body("Bad Google request or unable to proccess it");
}