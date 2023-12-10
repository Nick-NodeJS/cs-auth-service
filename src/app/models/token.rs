use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token_string: String,
    //Google keeps refresh_token until user revokes it, so None expire DateTime
    pub expire: Option<DateTime<Utc>>,
}
