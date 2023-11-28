use std::fmt::{self, Display};

use jsonwebtoken::DecodingKey;
use serde::{Deserialize, Serialize};

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
pub struct GoogleTokens {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: Option<String>,
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
