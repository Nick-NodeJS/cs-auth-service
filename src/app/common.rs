use actix_web::{error, Error, HttpRequest, HttpResponse};

use super::services::common::error_as_json;

pub mod api_path {
    pub const API: &str = "/api";
    pub const V1: &str = "/v1";
    pub const AUTH: &str = "/auth";
    pub const CYBER_SHERLOCK: &str = "/cyber-sherlock";
    pub const GOOGLE: &str = "/google";
    pub const FACEBOOK: &str = "/facebook";
    pub const LOGIN: &str = "/login";
    pub const CALLBACK: &str = "/callback";
    pub const LOGOUT: &str = "/logout";
    pub const STATUS: &str = "/status";
    pub const REGISTER: &str = "/register";
}

//TODO: implement default error handler

pub fn error_handler(err: actix_web_validator::Error, _: &HttpRequest) -> Error {
    let bs = format!("{}", &err);
    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(error_as_json(bs.as_ref())),
    )
    .into()
}
