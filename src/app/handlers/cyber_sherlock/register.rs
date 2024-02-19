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

/// return Google Auth URL as json
pub async fn register(
    app_data: web::Data<AppData>,
    query_data: Json<RegisterQueryData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    // TODO: implement error handler to have always json body format in response
    query_data.validate()?;

    // Check if user with the credentials exists
    let user_service = app_data.user_service.lock()?;
    if user_service
        .find_user_by_email_or_phone(&query_data.email, &query_data.phone)
        .await?
        .is_some()
    {
        return Ok(HttpResponse::BadRequest().json(error_as_json(USER_WITH_THE_CREDENTIALS_EXISTS)));
    }
    // Generate the authorization URL and params to verify it in next
    let mut cyber_sherlock_auth_provider = app_data.cyber_sherlock_auth_provider.lock()?;

    let auth_url =
        cyber_sherlock_auth_provider.get_authorization_url(&query_data, session.metadata)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(&auth_url)))
}
