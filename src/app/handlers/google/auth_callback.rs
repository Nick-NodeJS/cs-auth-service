use actix_web::{web, HttpRequest, HttpResponse};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    models::user::UserProfile,
    services::common::{error_as_json, tokens_as_json},
};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    let mut google_service = app_data.google_service.lock()?;
    let user_service = app_data.user_service.lock()?;
    let (code, state) = google_service.parse_auth_query_string(req.query_string())?;

    // process code and state to get tokens
    let tokens = google_service.get_tokens(code, state).await?;
    let user_profile = google_service.get_user_profile(&tokens.id_token).await?;

    // in case Google API returns no refresh token, it has to check if user was logged in before
    // if No(google refresh token is not in system) - it revoke the token and user has to relogin to Google
    // TODO: during adding a new or updating an existen user it should set session data(refresh token, login timestamp etc)
    if let Some(refresh_token) = tokens.refresh_token {
        user_service
            .create_or_update_user_with_profile(UserProfile::Google(user_profile))
            .await?;
        return Ok(HttpResponse::Ok().json(tokens_as_json((tokens.access_token, refresh_token))));
    } else {
        log::warn!(
            "\nUser id: {} google token response has no refresh token\n",
            user_profile.user_id
        );
        if let Some(google_refresh_token) = user_service
            .check_if_user_logged_in(user_profile.user_id.clone())
            .await?
        {
            user_service
                .create_or_update_user_with_profile(UserProfile::Google(user_profile))
                .await?;
            return Ok(HttpResponse::Ok()
                .json(tokens_as_json((tokens.access_token, google_refresh_token))));
        } else {
            log::warn!(
                "\nGoogle user id: {} has no refresh token. Should relogin\n",
                user_profile.user_id
            );
            google_service
                .revoke_token(tokens.access_token.clone())
                .await?;
            return Ok(
                HttpResponse::Unauthorized().json(error_as_json("User should relogin".to_string()))
            );
        }
    }
}
