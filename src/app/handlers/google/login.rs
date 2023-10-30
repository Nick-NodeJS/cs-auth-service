use cs_shared_lib::redis;
use serde_json::{Map, Value};
use crate::app::{app_data::AppData, app_error::AppError};

use actix_web::{web, HttpResponse};

/**
 * TODO:
 * - handle error with location
 * - implement Redis pool to do not restart service if redis down and than up
 */

pub async fn login(app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    let auth_url_payload = process_login(app_data).await?;
    Ok(HttpResponse::Ok().json(auth_url_payload))
}

async fn process_login(app_data: web::Data<AppData>) -> Result<Map<String, Value>, AppError> {
    let google_service = app_data.google_service.lock()?;
    // Generate the authorization URL to which we'll redirect the user.
    let (
        authorize_url,
        csrf_state,
        pkce_code_verifier,
        google_redis_state_ttl_ms,
    ) = google_service.get_authorization_url_data();
    let mut redis_connection = app_data.redis_connection.lock()?;
    redis::set_value_with_ttl(
        &mut redis_connection,
        csrf_state.secret().as_str(),
         &pkce_code_verifier,
          google_redis_state_ttl_ms as usize,
        )?;
    let mut result = Map::new();
    result.insert("authorize_url".to_string(), Value::String(authorize_url.to_string()));
    return Ok(result);
}