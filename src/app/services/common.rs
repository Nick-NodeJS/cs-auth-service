use awc::error::HeaderValue;
use oauth2::http::HeaderMap;
use serde_json::{json, Value};

/// return error string as json object
/// #### Arguments
///
/// * `error` - ```String```
///
/// #### Response example:
/// ```
///  {
///   "error": "Some error information"
///   }
/// ```
pub fn error_as_json(error: &str) -> Value {
    json!({ "error": error })
}

pub fn get_x_www_form_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}

pub fn auth_url_as_json(auth_url: &str) -> Value {
    json!({"authorization_url": auth_url})
}
