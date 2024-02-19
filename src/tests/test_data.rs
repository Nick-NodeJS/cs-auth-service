use bson::oid::ObjectId;

use crate::app::models::{
    common::AuthProviders,
    session::{NewSessionData, Session},
    session_metadata::SessionMetadata,
    session_tokens::SessionTokens,
    token::Token,
    user::User,
    user_profile::{GoogleProfile, UserProfile},
};

pub struct TestData {
    pub user: User,
    pub google_session: Session,
}

impl TestData {
    pub fn new() -> TestData {
        let mut google_profile = GoogleProfile {
            user_id: String::from("fake_google_user_id_"),
            name: String::from("fake_google_user_name"),
            email: String::from("fake_google_user_email@gmail.com"),
            email_verified: true,
            picture: Some(String::from("http://localhost")),
        };

        // We need to keep uniq Google user_id
        // it can catch wrong user by profile query in case equal google.user_id
        google_profile
            .user_id
            .push_str(ObjectId::new().to_string().as_ref());
        let user = User::new(UserProfile::Google(google_profile));

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
