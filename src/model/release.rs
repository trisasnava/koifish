use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    name: String,
    version: String,
    browser_download_url: String,
}
