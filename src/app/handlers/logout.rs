use actix_web::{web, HttpResponse};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    handlers::common::response::NO_USER_FOUND,
    models::{common::AuthProviders, session::Session},
    services::common::{error_as_json, result_as_json},
};

use super::common::response::SUCCESS;

pub async fn logout(
    app_data: web::Data<AppData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if !session.is_anonymous() {
        match session.auth_provider {
            AuthProviders::Google => {
                let google_provider = app_data.google_provider.lock()?;
                google_provider.logout(session.tokens.clone()).await?;
            }
            AuthProviders::Facebook => {
                let mut user_service = app_data.user_service.lock()?;
                match user_service.get_user_by_id(&session.user_id).await? {
                    Some(user) => {
                        let facebook_user = match user.facebook {
                            Some(facebook_profile) => facebook_profile,
                            None => {
                                log::debug!("No User Facebook Profile found on User: {:?} with active AuthProvider::Facebook, Session: {:?}", &user, &session);
                                return Ok(HttpResponse::InternalServerError()
                                    .json(error_as_json(NO_USER_FOUND)));
                            }
                        };
                        let mut facabook_provider = app_data.facebook_provider.lock()?;
                        facabook_provider.logout(&facebook_user.user_id).await?
                    }
                    None => {
                        log::debug!("No User found by Session {:?}", &session);
                        return Ok(
                            HttpResponse::InternalServerError().json(error_as_json(NO_USER_FOUND))
                        );
                    }
                };
            }
            _ => {}
        }
        let mut user_service = app_data.user_service.lock()?;
        user_service.logout_by_session(session).await?;
    }
    Ok(HttpResponse::Ok().json(result_as_json(SUCCESS)))
}
