use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::app::{app_data::AppData, app_error::AppError, models::session::Session};

pub async fn logout(
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    // dbg!("User session from request:", &session);
    if !session.is_anonymous() {
        match session.auth_provider {
            // TODO: on every auth_provider option implement logout and execut it here
            _ => {}
        }
        let mut user_service = app_data.user_service.lock()?;
        user_service.logout_by_session(session).await?;
    }
    Ok(HttpResponse::Ok().json(json!({"result": "successful"})))
}
