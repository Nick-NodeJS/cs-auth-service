use actix_web::cookie::{time::Duration, Key, SameSite};
use cs_shared_lib::validation::validate_integer_in_range;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub enum CookieContentSecurity {
    /// The cookie content is encrypted when using `CookieContentSecurity::Private`.
    ///
    /// Encryption guarantees confidentiality and integrity: the client cannot tamper with the
    /// cookie content nor decode it, as long as the encryption key remains confidential.
    Private,

    /// The cookie content is signed when using `CookieContentSecurity::Signed`.
    ///
    /// Signing guarantees integrity, but it doesn't ensure confidentiality: the client cannot
    /// tamper with the cookie content, but they can read it.
    Signed,
}

#[derive(Clone)]
pub struct CookieConfiguration {
    pub secure: bool,
    pub http_only: bool,
    pub name: String,
    pub same_site: SameSite,
    pub path: String,
    pub domain: Option<String>,
    pub max_age: Option<Duration>,
    pub content_security: CookieContentSecurity,
    pub key: Key,
}

#[derive(Clone)]
pub struct SessionConfig {
    pub cookie_config: CookieConfiguration,
    pub session_ttl_sec: i64,
}

impl SessionConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let session_ttl_sec = dotenv::var("SESSION_TTL_SEC")
            .expect("SESSION_TTL_SEC environment variable is not set")
            .parse()
            .expect("Invalid SESSION_TTL_SEC");

        // Session ttl in range (1 min - 30 days)
        if !validate_integer_in_range(session_ttl_sec, 60, 30 * 24 * 60 * 60) {
            panic!("Session ttl out of the range(1 min - 30 days)");
        }

        let cookie_key = dotenv::var("SESSION_COOKIE_KEY")
            .expect("Missing the SESSION_COOKIE_KEY environment variable.");

        Self {
            cookie_config: CookieConfiguration {
                secure: true,
                http_only: true,
                name: "id".into(),
                same_site: SameSite::Lax,
                path: "/".into(),
                domain: None,
                max_age: None,
                content_security: CookieContentSecurity::Private,
                key: Key::from(cookie_key.as_bytes()),
            },
            session_ttl_sec,
        }
    }
}
