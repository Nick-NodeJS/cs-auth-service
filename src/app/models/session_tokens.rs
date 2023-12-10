use serde::{Deserialize, Serialize};

use super::{common::AuthProviders, token::Token};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionTokens {
    pub access_token: Option<Token>,
    pub refresh_token: Option<Token>,
    // Google use id_token as JWT access tokento but id_token to provide access to its API, we set it like value of extra_token
    pub extra_token: Option<Token>,
}

impl SessionTokens {
    pub fn update_tokens(&mut self, tokens: SessionTokens) -> Self {
        if let Some(access_token) = tokens.access_token {
            self.access_token = Some(access_token);
        }
        if let Some(refresh_token) = tokens.refresh_token {
            self.refresh_token = Some(refresh_token);
        }
        if let Some(extra_token) = tokens.extra_token {
            self.extra_token = Some(extra_token);
        }
        self.to_owned()
    }
    pub fn is_incompleted(&self, provider: &AuthProviders) -> bool {
        // At this time Google only has extra token but in case user logged in
        // and tries to login again(for example from another device) it returns
        // tokens without refresh token, so we need to be able check if tokens
        // are completed
        match provider {
            AuthProviders::CyberSherlock => {
                self.access_token.is_some()
                    && self.refresh_token.is_some()
                    && self.extra_token.is_some()
            }
            _ => self.access_token.is_some() && self.refresh_token.is_some(),
        }
    }
}
