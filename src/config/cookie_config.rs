use std::fmt;

use actix_web::cookie::{time::Duration, Key, SameSite};

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

impl fmt::Debug for CookieConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CookieConfiguration:\nsecure: {},\nhttp_only:{},\nname: {},\nsame_site: {},\npath: {},\ndomain: {:?},\nmax_age: {:?},\ncontent_security: {:?}\n",
            self.secure,
            self.http_only,
            self.name,
            self.same_site,
            self.path,
            self.domain,
            self.max_age,
            self.content_security,
        )
    }
}
