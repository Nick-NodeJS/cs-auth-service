use crate::app::{
    app_data::AppData, app_error::AppError, models::session::Session,
    providers::cyber_sherlock::common::LoginQueryData, services::common::auth_url_as_json,
};
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

/// return Google Auth URL as json
pub async fn login(
    app_data: web::Data<AppData>,
    login_query_data: Json<LoginQueryData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    // TODO: implement error handler to have always json body format in response
    login_query_data.validate()?;
    // Generate the authorization URL and params to verify it in next
    let mut cyber_sherlock_auth_provider = app_data.cyber_sherlock_auth_provider.lock()?;

    let auth_url =
        cyber_sherlock_auth_provider.get_authorization_url(&login_query_data, session.metadata)?;

    Ok(HttpResponse::Ok().json(auth_url_as_json(&auth_url)))
}
