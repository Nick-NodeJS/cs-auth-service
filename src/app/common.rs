// use std::rc::Rc;

// use actix_web::dev::ServiceResponse;
// use actix_web::middleware::ErrorHandlerResponse;
// use actix_web::{
//     dev, error, http, web, App, Error, HttpRequest, HttpResponse, ResponseError, Result,
// };
// use awc::{ClientResponse, MessageBody, ResponseBody};
// use futures::FutureExt;
// use serde_json::{json, Value};

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
    pub const REGISTER: &str = "/register";
}

//TODO: implement default error handler

// pub fn default_error_handler<B>(
//     mut res: dev::ServiceResponse<B>,
// ) -> Result<ErrorHandlerResponse<B>> {
//     res.response_mut().headers_mut().insert(
//         http::header::CONTENT_TYPE,
//         http::header::HeaderValue::from_static("Service Error"),
//     );
//     //res = HttpResponse::BadRequest().json(error_as_json("bad request"));
//     let (req, res) = res.into_parts();
//     // let (_, body) = res.into_parts();
//     // let message_body = ResponseBody::(body);
//     let res = HttpResponse::BadRequest().json(json!({"error": "error message"}));
//     let res = ServiceResponse::new(req, res).map_into_left_body();

//     //ErrorHandlerResponse::Response(res)
//     //error::InternalError::from_response(Err("Fuck!!!"), res)
//     error::InternalError::from_response("dsfzsd", HttpResponse::Conflict().finish()).into()
// }

// |res| {
//     let (req, res) = res.into_parts();
//     let res = res.set_body(json!({"error":"error message"}));
//     let res = ServiceResponse::new(req, res);
//     Ok(actix_web::middleware::ErrorHandlerResponse::Response(res))
// }
