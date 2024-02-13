use crate::app::{providers::error::ProviderError, services::common::AsyncFn};

use super::common::{EMAIL, MOBILE};

pub struct NotificationProvider {
    async_http_request: Box<dyn AsyncFn>,
}

impl NotificationProvider {
    pub fn new(request: Box<dyn AsyncFn>) -> Self {
        NotificationProvider {
            async_http_request: request,
        }
    }

    pub fn send_email(&mut self, message: &str, email: EMAIL) -> Result<(), ProviderError> {
        log::debug!("Sending message {} to email {}", message, email);
        Ok(())
    }

    pub fn send_mobile(&mut self, message: &str, mobile: MOBILE) -> Result<(), ProviderError> {
        log::debug!("Sending message {} to mobile {}", message, mobile);
        Ok(())
    }
}
