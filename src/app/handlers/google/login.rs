use crate::app::{
    app_data::AppData, app_error::AppError, models::session::Session,
    providers::common::LoginCacheData, services::common::auth_url_as_json,
};
use actix_web::{web, HttpResponse};

/// return Google Auth URL as json
pub async fn login(
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    // Generate the authorization URL and params to verify it in next
    let mut google_provider = app_data.google_provider.lock()?;
    let (authorize_url, csrf_state, pkce_code_verifier) =
        google_provider.get_authorization_url_data();

    // set auth data in cache
    let login_cache_data = LoginCacheData {
        pkce_code_verifier,
        session,
    };
    google_provider.set_auth_data_to_cache(csrf_state.secret().as_ref(), &login_cache_data)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(authorize_url.as_ref())))
}
