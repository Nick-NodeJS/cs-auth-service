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
