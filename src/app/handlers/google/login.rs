use crate::app::{
    app_data::AppData,
    app_error::AppError,
    models::session_metadata::SessionMetadata,
    services::{common::auth_url_as_json, google::common::LoginCacheData},
};
use actix_web::{web, HttpRequest, HttpResponse};

/**
 * TODO:
 * - handle error with location
 */

/// return Google Auth URL as json
pub async fn login(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    // TODO: set session metadata from this place because the Google callbacl call doesn't have it(should be checked)
    let mut session_metadata = SessionMetadata::new();
    session_metadata.set_metadata_from_request(&req);
    // Generate the authorization URL and params to verify it in next
    let mut google_service = app_data.google_service.lock()?;
    let (authorize_url, csrf_state, pkce_code_verifier) =
        google_service.get_authorization_url_data();

    // set auth data in cache
    let login_cache_data = LoginCacheData {
        pkce_code_verifier,
        session_metadata,
    };
    google_service.set_auth_data_to_cache(csrf_state.secret().as_ref(), &login_cache_data)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(authorize_url.as_ref())))
}
