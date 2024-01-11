use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    models::{common::AuthProviders, session::Session},
    services::cache::service::RedisCacheService,
};

pub async fn logout(
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if !session.is_anonymous() {
        match session.auth_provider {
            AuthProviders::Google => {
                let google_service = app_data.google_service.lock()?;
                google_service.logout(session.tokens.clone()).await?;
            }
            _ => {}
        }
        let mut user_service = app_data.user_service.lock()?;
        user_service.logout_by_session(session).await?;
    }
    Ok(HttpResponse::Ok().json(json!({"result": "successful"})))
}
