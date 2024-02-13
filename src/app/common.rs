// use actix_web::middleware::ErrorHandlerResponse;
// use actix_web::{dev, http, web, App, HttpRequest, HttpResponse, Result};
// use awc::{ClientResponse, ResponseBody};

// use super::services::common::error_as_json;

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
}

// pub fn bad_request_error_handler<B>(
//     mut res: dev::ServiceResponse<B>,
// ) -> Result<ErrorHandlerResponse<B>> {
//     res.response_mut().headers_mut().insert(
//         http::header::CONTENT_TYPE,
//         http::header::HeaderValue::from_static("Error"),
//     );
//     //res = HttpResponse::BadRequest().json(error_as_json("bad request"));
//     Ok(ErrorHandlerResponse::Response(res))
// }
