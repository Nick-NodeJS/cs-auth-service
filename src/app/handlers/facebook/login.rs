use crate::app::{
    app_data::AppData, app_error::AppError, models::session_metadata::SessionMetadata,
    services::common::auth_url_as_json,
};
use actix_web::{web, HttpRequest, HttpResponse};

/// return Google Auth URL as json
pub async fn login(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    // TODO: set session metadata from this place because the Google callbacl call doesn't have it(should be checked)
    let mut session_metadata = SessionMetadata::new();
    session_metadata.set_metadata_from_request(&req);
    // Generate the authorization URL and params to verify it in next
    let mut facebook_provider = app_data.facebook_provider.lock()?;

    let auth_url = facebook_provider.get_authorization_url_data(session_metadata)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(&auth_url)))
}
