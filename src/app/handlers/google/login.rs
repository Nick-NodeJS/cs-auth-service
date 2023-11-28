use crate::app::{app_data::AppData, app_error::AppError};
use actix_web::{web, HttpResponse};
use serde_json::{Map, Value};

/**
 * TODO:
 * - handle error with location
 */

/// return Google Auth URL as json
pub async fn login(app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    // Generate the authorization URL and params to verify it in next
    let mut google_service = app_data.google_service.lock()?;
    let (authorize_url, csrf_state, pkce_code_verifier) =
        google_service.get_authorization_url_data();

    // set auth data in cache

    google_service.set_auth_data_to_cache(csrf_state.secret().as_ref(), &pkce_code_verifier)?;

    // make and return json auth url payload
    let mut auth_url_payload = Map::new();
    auth_url_payload.insert(
        "authorize_url".to_string(),
        Value::String(authorize_url.to_string()),
    );
    Ok(HttpResponse::Ok().json(auth_url_payload))
}
