use crate::app::models::{
    common::AuthProviders,
    session::{NewSessionData, Session},
    session_metadata::SessionMetadata,
    session_tokens::SessionTokens,
    token::Token,
    user::{GoogleProfile, User, UserProfile},
};

pub struct TestData {
    pub user: User,
    pub google_session: Session,
}

impl TestData {
    pub fn new() -> TestData {
        let user = User::new(UserProfile::Google(GoogleProfile {
            user_id: String::from("fake_google_user_id"),
            name: String::from("fake_google_user_name"),
            email: String::from("fake_google_user_email@gmail.com"),
            email_verified: true,
            picture: String::from("http://localhost"),
        }));
        TestData {
            user: user.clone(),
            google_session: Session::new(NewSessionData {
                anonimous: false,
                auth_provider: AuthProviders::Google,
                user_id: user.id.clone(),
                tokens: SessionTokens {
                    access_token: Some(Token {
                        token_string: String::from("fake_google_id_token_token"),
                        expire: None,
                    }),
                    refresh_token: Some(Token {
                        token_string: String::from("fake_google_refresh_token_token"),
                        expire: None,
                    }),
                    extra_token: Some(Token {
                        token_string: String::from("fake_google_access_token_token"),
                        expire: None,
                    }),
                },
                session_metadata: SessionMetadata::new(),
            }),
        }
    }
}
