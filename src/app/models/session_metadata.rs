use actix_web::{http::header, HttpRequest};
use serde::{Deserialize, Serialize};

const UNKNOWN_DATA: &str = "unknown";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMetadata {
    pub user_agent: String,
    pub user_host: String,
}

impl SessionMetadata {
    pub fn new() -> Self {
        SessionMetadata {
            user_agent: UNKNOWN_DATA.to_string(),
            user_host: UNKNOWN_DATA.to_string(),
        }
    }
    pub fn set_metadata_from_request(&mut self, request: &HttpRequest) -> () {
        let mut head = request.head().clone();
        if let Some(user_agent_value) = head.headers_mut().get(header::USER_AGENT) {
            match user_agent_value.to_str() {
                Ok(user_agent) => self.user_agent = user_agent.to_string(),
                Err(err) => {
                    log::debug!(
                        "Unable to get USER_AGENT string from : {:?}, error: {}",
                        user_agent_value,
                        err
                    );
                }
            };
        }
        if let Some(user_host_value) = head.headers_mut().get(header::HOST) {
            match user_host_value.to_str() {
                Ok(user_host) => self.user_host = user_host.to_string(),
                Err(err) => {
                    log::debug!(
                        "Unable to get HOST string from : {:?}, error: {}",
                        user_host_value,
                        err
                    );
                }
            };
        }
    }
}
