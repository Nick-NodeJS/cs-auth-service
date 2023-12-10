use actix_web::{http::header, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMetadata {
    pub user_agent: Option<String>,
}

impl SessionMetadata {
    pub fn new() -> Self {
        SessionMetadata { user_agent: None }
    }
    pub fn set_metadata_from_request(&mut self, request: &HttpRequest) -> () {
        let mut head = request.head().clone();
        if let Some(user_agent_value) = head.headers_mut().get(header::USER_AGENT) {
            self.user_agent = match user_agent_value.to_str() {
                Ok(user_agent) => Some(user_agent.to_string()),
                Err(err) => {
                    log::debug!(
                        "Unable to get USER_AGENT string from : {:?}, error: {}",
                        user_agent_value,
                        err
                    );
                    None
                }
            };
        }
    }
}
