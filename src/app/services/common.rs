use awc::error::HeaderValue;
use chrono::Utc;
use oauth2::reqwest;
use oauth2::reqwest::AsyncHttpClientError;
use oauth2::HttpRequest;
use oauth2::{http::HeaderMap, HttpResponse};
use reqwest::async_http_client;
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

pub fn result_as_json(result: &str) -> Value {
    json!({ "result": result })
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

pub async fn async_http_request(
    request: HttpRequest,
) -> Result<HttpResponse, AsyncHttpClientError> {
    let start = Utc::now();
    let response = async_http_client(request.clone()).await?;
    let finish = Utc::now();
    let result_string: &str;
    if !response.status_code.is_success() {
        result_string = "Fail";
    } else {
        result_string = "Success";
    }
    log::debug!(
        "\n{}!!! \nRequest: {:?} \nExecution\n start: {}\nfinish: {}\nResponse body: {:?}\n, body as string: {}",
        result_string,
        request,
        &start,
        &finish,
        &response.body,
        String::from_utf8_lossy(&response.body)
    );
    Ok(response)
}
