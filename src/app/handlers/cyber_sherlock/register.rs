use crate::app::{
    app_data::AppData,
    app_error::AppError,
    handlers::common::response::USER_WITH_THE_CREDENTIALS_EXISTS,
    models::session::Session,
    providers::cyber_sherlock::common::RegisterQueryData,
    services::common::{auth_url_as_json, error_as_json},
};
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

/// return CyberSherlock Auth URL as json
pub async fn register(
    app_data: web::Data<AppData>,
    query_data: Json<RegisterQueryData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    //TODO: check if it possible validate on request query parsing step
    query_data.validate()?;

    // Check if user with the credentials exists
    let user_service = app_data.user_service.lock()?;
    if user_service
        .find_user_by_credentials(&query_data.to_credentials())
        .await?
        .is_some()
    {
        return Ok(HttpResponse::BadRequest().json(error_as_json(USER_WITH_THE_CREDENTIALS_EXISTS)));
    }
    // Generate the authorization URL and params to verify it in next
    let mut cyber_sherlock_auth_provider = app_data.cyber_sherlock_auth_provider.lock()?;

    let auth_url = cyber_sherlock_auth_provider.get_authorization_url(&query_data, session)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(&auth_url)))
}
