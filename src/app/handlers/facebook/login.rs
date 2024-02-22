use crate::app::{
    app_data::AppData, app_error::AppError, models::session::Session,
    services::common::auth_url_as_json,
};
use actix_web::{web, HttpResponse};

/// return Google Auth URL as json
pub async fn login(
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    // Generate the authorization URL and params to verify it in next
    let mut facebook_provider = app_data.facebook_provider.lock()?;

    let auth_url = facebook_provider.get_authorization_url_data(session)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(&auth_url)))
}
