use actix_web::{web, HttpResponse};

use crate::app::{app_data::AppData, app_error::AppError};

pub async fn logout(app_data: web::Data<AppData>) -> Result<HttpResponse, AppError> {
    // TODO: logout on provider and then(revoke token)
    // remove all user sessions for the session provider
    // - get session_token from cookie
    // - decode cookie to get session_key
    // - get session by session_key
    // - logout on session provider
    // - remove all session's provider sessions on session's user
    // - update user session key array in cache
    Ok(HttpResponse::Ok().body("Google logout"))
}
