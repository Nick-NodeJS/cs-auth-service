use actix_web::{HttpRequest, HttpResponse, web};

use crate::app::{app_data::AppData, app_error::AppError};

pub async fn auth_callback(req: HttpRequest, app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
  let mut redis_service = app_data.redis_service.lock()?;
  let google_service = &app_data.google_service.lock()?;
  let (code, state) = google_service.parse_query_string(req.query_string())?;
  // process code and state
  let try_code: Option<String> = redis_service.get_value(state.clone().as_ref())?;
  if let Some(pkce_code_verifier) = try_code {
    let tokens = google_service.get_tokens(code, pkce_code_verifier).await?;
    return Ok(
      HttpResponse::Ok()
      .json(google_service.tokens_as_json(tokens))
    );
  } else {
    log::debug!("No callback request state {} in Redis", state);
    return Err(AppError::CallbackStateCacheError);
  }
}