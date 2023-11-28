use awc::error::HeaderValue;
use oauth2::http::HeaderMap;
use serde_json::{Map, Value};

/// return tokens as json object
/// #### Arguments
///
/// * `tokens` - A Tuple of strings
///
/// ```
/// (String, String)
/// ```
///
/// where tokens\[0\] is access_token and tokens\[1\] is refresh_token
///
/// #### Response example:
/// ```
///  {
///   "access_token": "$access_token",
///   "refresh_token": "$refresh_token"
///   }
/// ```
pub fn tokens_as_json(tokens: (String, String)) -> Map<String, Value> {
    let (access_token, refresh_token) = tokens;
    let mut payload = Map::new();
    payload.insert("access_token".to_string(), Value::String(access_token));
    payload.insert("refresh_token".to_string(), Value::String(refresh_token));
    return payload;
}

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
pub fn error_as_json(error: String) -> Map<String, Value> {
    let mut payload = Map::new();
    payload.insert("error".to_string(), Value::String(error));
    return payload;
}

pub fn get_x_www_form_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers
}
