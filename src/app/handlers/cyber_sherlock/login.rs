use crate::app::{
    app_data::AppData,
    app_error::AppError,
    handlers::common::response::{NO_USER_FOUND, SUCCESS, WRONG_CREDENTIALS},
    models::{
        common::AuthProviders,
        session::{NewSessionData, Session},
    },
    providers::cyber_sherlock::common::{verify_password, LoginQueryData},
    services::common::{error_as_json, result_as_json},
};
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

//TODO: check if session is not anonymous
/// return Google Auth URL as json
pub async fn login(
    app_data: web::Data<AppData>,
    login_query_data: Json<LoginQueryData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    login_query_data.validate()?;
    // Find user by login data with CyberSherlock profile
    let mut user_service = app_data.user_service.lock()?;
    if let Some(user) = user_service
        .find_user_by_email_or_phone(&login_query_data.email, &login_query_data.phone)
        .await?
    {
        // validate login data password by profile hash
        if let Some(cyber_sherlock_profile) = user.cyber_sherlock {
            return match verify_password(&login_query_data.password, &cyber_sherlock_profile.hash) {
                Ok(_) => {
                    let cyber_sherlock_auth_provider =
                        app_data.cyber_sherlock_auth_provider.lock()?;
                    let tokens =
                        cyber_sherlock_auth_provider.get_tokens(&cyber_sherlock_profile)?;
                    let new_session = user_service
                        .set_new_session(NewSessionData {
                            anonimous: false,
                            auth_provider: AuthProviders::CyberSherlock,
                            user_id: user.id,
                            tokens,
                            session_metadata: session.metadata,
                        })
                        .await?;
                    let mut response = HttpResponse::Ok().json(result_as_json(SUCCESS));
                    user_service.set_session_cookie(response.head_mut(), &new_session)?;

                    Ok(response)
                }
                Err(err) => {
                    log::debug!("CyberSherlock login Error: {}", err);
                    Ok(HttpResponse::Unauthorized().json(error_as_json(WRONG_CREDENTIALS)))
                }
            };
        };
        return Ok(HttpResponse::Unauthorized().json(error_as_json(WRONG_CREDENTIALS)));
    }
    return Ok(HttpResponse::Unauthorized().json(error_as_json(NO_USER_FOUND)));
}
