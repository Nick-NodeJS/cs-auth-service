use serde_json::{Map, Value};
use crate::app::{app_data::AppData, app_error::AppError};
use actix_web::{web, HttpResponse};

/**
 * TODO:
 * - handle error with location
 */

/// return Google Auth URL as json
pub async fn login(app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    // Generate the authorization URL and params to verify it in next
    let google_service = app_data.google_service.lock()?;
    let (
        authorize_url,
        csrf_state,
        pkce_code_verifier,
        google_redis_state_ttl_ms,
    ) = google_service.get_authorization_url_data();

    // get redis service and set auth data in cache
    let mut redis_service = app_data.redis_service.lock()?;
    redis_service
        .set_value_with_ttl(
        csrf_state.secret().as_str(),
         &pkce_code_verifier,
          google_redis_state_ttl_ms as usize,
        )?;

    // make and return json auth url payload
    let mut auth_url_payload = Map::new();
    auth_url_payload.insert("authorize_url".to_string(), Value::String(authorize_url.to_string()));
    Ok(HttpResponse::Ok().json(auth_url_payload))
}