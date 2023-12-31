use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};

use crate::app::{
    app_data::AppData,
    app_error::AppError,
    models::user::UserProfile,
    services::common::{error_as_json, result_as_json},
};

pub async fn auth_callback(
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
    let mut google_service = app_data.google_service.lock()?;
    let mut user_service = app_data.user_service.lock()?;
    let (code, state) = google_service.parse_auth_query_string(req.query_string())?;

    let login_cache_data = google_service.get_pkce_code_verifier(&state)?;
    // process code and state to get tokens
    let tokens = google_service
        .get_tokens(code, login_cache_data.pkce_code_verifier)
        .await?;

    let user_profile = google_service
        .get_user_profile(tokens.access_token.clone())
        .await?;

    if let Some(user_session) = user_service
        .get_user_session(
            tokens.clone(),
            UserProfile::Google(user_profile.clone()),
            login_cache_data.session_metadata,
        )
        .await?
    {
        log::debug!(
            "User {} loged in with Google successfuly",
            user_session.user_id
        );
        // TODO: set session token to cookie
        let mut response = HttpResponse::Ok().json(result_as_json("success"));
        user_service.set_session_cookie(response.head_mut(), user_session.id)?;

        Ok(response)
    } else {
        log::warn!(
            "\nGoogle user_id: {} has no data in system. Should relogin\n",
            user_profile.user_id
        );
        if let Some(token) = tokens.extra_token {
            google_service
                .revoke_token(token.token_string.as_ref())
                .await?;
        }
        // TODO: investigate if it's better for UX to pass throw login to return auth_url on this step
        return Ok(
            HttpResponse::Unauthorized().json(error_as_json("User should relogin to Google"))
        );
    }
}
