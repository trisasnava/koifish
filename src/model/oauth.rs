use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct OauthToken {
    token: String,
}

impl OauthToken {
    pub fn new(str: &str) -> Self {
        Self {
            token: str.parse().unwrap(),
        }
    }

    pub fn value(&self) -> &str {
        &self.token
    }
}
