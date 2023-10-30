use redis::Commands;

use actix_web::{HttpRequest, HttpResponse, web};

use crate::app::app_data::AppData;

pub async fn auth_callback(req: HttpRequest, app_data: web::Data<AppData>) -> HttpResponse {
  let mut redis_connection = match app_data.redis_connection.lock() {
    Ok(connection) => connection,
    Err(err) => return google_bad_request_error(err.to_string()),
  };
  let google_service_lock = &app_data.google_service.lock();
  let google_service = match google_service_lock {
    Ok(mutex_google_service) => mutex_google_service,
    Err(err) => return google_bad_request_error(err.to_string()),
  };
  match google_service.parse_query_string(req.query_string()) {
      Ok((code, state)) => {
        // process code + state
        let try_code: Option<String> = match redis_connection.get(state.clone()) {
          Ok(cache_code) => cache_code,
          Err(err) => return google_bad_request_error(err.to_string()),
        };
        if let Some(pkce_code_verifier) = try_code {
          match google_service.get_tokens(code, pkce_code_verifier).await {
            Ok(tokens) => {
              if let Err(err) = google_service.set_user_to_storage(&tokens/*, redis_connection*/).await {
                return google_bad_request_error(
                  format!("Error to set tokens to storage: {}",err)
                )
              }
              return HttpResponse::Ok().json(google_service.tokens_as_json(tokens));
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

// TODO: optimize it
fn google_bad_request_error(err: String) -> HttpResponse {
  log::error!("Bad Google request: {}", err);
  return HttpResponse::BadRequest().body("Bad Google request or unable to proccess it");
}