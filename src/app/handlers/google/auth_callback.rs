use actix_web::{HttpRequest, HttpResponse, web};

use crate::app::{app_data::AppData, app_error::AppError, services::user::user::GoogleProfile};

pub async fn auth_callback(req: HttpRequest, app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
  let mut redis_service = app_data.redis_service.lock()?;
  let google_service = app_data.google_service.lock()?;
  let (code, state) = google_service.parse_query_string(req.query_string())?;
  // process code and state
  let try_code: Option<String> = redis_service.get_value(state.clone().as_ref())?;
  let pkce_code_verifier: String;
  if let Some(pkce_code_verifier_from_cache) = try_code {
    pkce_code_verifier = pkce_code_verifier_from_cache;
  } else {
    log::debug!("No callback request state {} in Redis", state);
    return Err(AppError::CallbackStateCacheError);
  }
  // TODO: google_service.get_user().await,
  // user_service.set_google_user().await, including data storage and cache updating
  let tokens = google_service.get_tokens(code, pkce_code_verifier).await?;
  let token_data = google_service.get_user_profile(tokens.clone()).await?;
  // let google_profile = GoogleProfile(token_data);
  let tokens_as_json = google_service.tokens_as_json(tokens);
  Ok(HttpResponse::Ok().json(tokens_as_json))
}