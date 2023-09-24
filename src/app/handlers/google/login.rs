use serde_json::{Map, Value};
use crate::app::app_data::AppData;

use actix_web::{web, HttpResponse};

pub async fn login(app_data: web::Data<AppData>) -> HttpResponse {
    let google_service = &app_data.google_service.lock().unwrap();
    // Generate the authorization URL to which we'll redirect the user.
    let (
        authorize_url,
        csrf_state,
        pkce_code_verifier,
        google_redis_state_ttl_ms,
    ) = google_service.get_authorization_url_data();

    // Set pkce_code_verifier to Redis by key as csrf_state
    let redis_service = &app_data.redis_service.lock().unwrap();
    if let Err(err) = redis_service.set_value_with_ttl(
        csrf_state.secret().as_str(),
         &pkce_code_verifier,
          google_redis_state_ttl_ms as usize,
        ).await {
            log::error!("REDIS SERVICE ERROR: {}", err);
            return HttpResponse::InternalServerError().body("Service unavailable")//Err(actix_web::error::ErrorInternalServerError(e));
    }

    // Redirect the user to the Google OAuth2 authorization page
    let mut payload = Map::new();
    payload.insert("authorize_url".to_string(), Value::String(authorize_url.to_string()));
    HttpResponse::Ok().json(payload)//.body(authorize_url.to_string())
}