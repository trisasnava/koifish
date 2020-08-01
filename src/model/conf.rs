use serde::Deserialize;
use serde::Serialize;

use crate::model::oauth::OauthToken;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    oauth_token: Option<OauthToken>,
}

impl Config {
    pub fn get_token(&self) -> &str {
        match &self.oauth_token {
            Some(token) => token.value().as_str(),
            None => "",
        }
    }
}
