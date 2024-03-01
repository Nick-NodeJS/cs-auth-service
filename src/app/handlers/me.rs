use actix_web::{web, HttpResponse};

use crate::app::{
    app_data::AppData, app_error::AppError, handlers::common::response::NO_USER_FOUND,
    models::session::Session, services::common::error_as_json,
};

// TODO: implement Auth Guard
pub async fn me(app_data: web::Data<AppData>, session: Session) -> Result<HttpResponse, AppError> {
    let mut user_service = app_data.user_service.lock()?;
    match user_service.get_user_by_id(&session.user_id).await? {
        None => Ok(HttpResponse::BadRequest().json(error_as_json(NO_USER_FOUND))),
        Some(user) => Ok(HttpResponse::Ok().json(user.to_json())),
    }
}
