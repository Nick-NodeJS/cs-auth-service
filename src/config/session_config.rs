use actix_web::cookie::{Key, SameSite};
use cs_shared_lib::validation::validate_integer_in_range;
use dotenv::dotenv;

use super::cookie_config::{CookieConfiguration, CookieContentSecurity};

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub cookie_config: CookieConfiguration,
    pub session_ttl_sec: u64,
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
