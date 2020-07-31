use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct OauthToken {
    token: String,
}

impl OauthToken {
    pub fn new(str: String) -> Self {
        Self { token: str }
    }

    pub fn value(&self) -> &String {
        &self.token
    }
}