use std::fmt::{self};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::app::{
    models::{session_tokens::SessionTokens, token::Token},
    providers::error::ProviderError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenHeaderObject {
    pub kid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoogleKeys {
    pub keys: Vec<GoogleCert>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GoogleCert {
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

struct DisplayGoogleCerts<'a>(&'a Vec<GoogleCert>);

impl<'a> fmt::Display for DisplayGoogleCerts<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cert in self.0.iter() {
            writeln!(f, "\n{:?}", cert)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub scope: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub at_hash: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub iat: u32,
    pub exp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
}

pub fn get_session_tokens(tokens: GoogleTokenResponse) -> SessionTokens {
    let expire = Some(Utc::now() + Duration::seconds(tokens.expires_in));
    let refresh_token = match tokens.refresh_token {
        Some(token) => Some(Token {
            token_string: token,
            expire: None,
        }),
        None => None,
    };
    SessionTokens {
        access_token: Some(Token {
            token_string: tokens.id_token,
            expire,
        }),
        refresh_token,
        extra_token: Some(Token {
            token_string: tokens.access_token,
            expire,
        }),
    }
}

pub fn decode_token(
    token: &str,
    key: &DecodingKey,
    check_expiration: bool,
) -> Result<TokenClaims, ProviderError> {
    // Validation configuration
    let mut validation = Validation::new(Algorithm::RS256);
    if !check_expiration {
        validation.validate_exp = false;
    }

    let token_data: TokenData<TokenClaims> = match decode(token, key, &validation) {
        Ok(data) => data,
        Err(err) => {
            log::warn!("Decode Error: {}\n token: {}\n", err, token);
            return Err(ProviderError::JWTDecodingError);
        }
    };

    Ok(token_data.claims)
}

pub fn get_decoding_key_from_vec_cert(
    certs: Vec<GoogleCert>,
    kid: String,
) -> Result<Option<DecodingKey>, ProviderError> {
    let cert = certs.clone().into_iter().find(|c| c.kid == kid);
    if let Some(certificate) = cert {
        let key = DecodingKey::from_rsa_components(&certificate.n, &certificate.e)?;
        Ok(Some(key.clone()))
    } else {
        log::error!(
            "No certificate found in cache for kid: {}. Certificates: {:?}",
            kid,
            &certs
        );
        Ok(None)
    }
}
