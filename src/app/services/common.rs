use awc::error::HeaderValue;
use chrono::Utc;
use futures::future::BoxFuture;
use futures::Future;
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

pub trait AsyncFn: Send {
    fn handle(
        &mut self,
        args: HttpRequest,
    ) -> BoxFuture<'static, Result<HttpResponse, AsyncHttpClientError>>;
}

impl<T, F> AsyncFn for T
where
    T: FnMut(HttpRequest) -> F + Send,
    F: Future<Output = Result<HttpResponse, AsyncHttpClientError>> + 'static + Send,
{
    fn handle(
        &mut self,
        args: HttpRequest,
    ) -> BoxFuture<'static, Result<HttpResponse, AsyncHttpClientError>> {
        Box::pin(self(args))
    }
}

pub async fn async_http_request(
    request: HttpRequest,
) -> Result<HttpResponse, AsyncHttpClientError> {
    let start = Utc::now();
    let response = async_http_client(request.clone()).await?;
    let finish = Utc::now();
    let mut message_string: String;
    if !response.status_code.is_success() {
        message_string = String::from("\nFail!!!\n");
    } else {
        message_string = String::from("\nSuccess!!!\n");
    }
    message_string.push_str(
        format!(
            "Request: {:#?} \nExecution\n start: {}\nfinish: {}\nResponse body:",
            request, &start, &finish,
        )
        .as_ref(),
    );
    let body_as_json = match serde_json::from_slice(&response.body) {
        Ok(json) => json,
        Err(_) => json!({
                "BODY_AS_A_STRING": String::from_utf8_lossy(&response.body)
        }),
    };
    message_string.push_str(format!("{:#?}\n", body_as_json).as_ref());
    log::debug!("{message_string}");
    Ok(response)
}
