use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Oauth {
    pub token: Option<String>,
    pub platform: Option<String>,
    pub datetime: Option<String>,
}

impl Oauth {
    fn new(platform: &str, token: &str) -> Self {
        Self {
            token: Option::from(token.to_string()),
            platform: Option::from(platform.to_string()),
            datetime: Option::from(chrono::offset::Local::now().to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub oauth: Oauth,
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let path_buf = Path::new(dirs::home_dir().unwrap().as_path()).join(".koi");
        Self {
            path: path_buf,
            oauth: Oauth {
                platform: None,
                token: None,
                datetime: None,
            },
        }
    }

    pub fn read(&self) -> std::io::Result<Self> {
        if !&self.path.exists() {
            File::create(&self.path.as_path())?;
        }

        let mut file = File::open(&self.path.as_path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Oauth = toml::from_str(contents.as_str()).unwrap();

        Ok(Self {
            path: Default::default(),
            oauth: config,
        })
    }

    /// Save GitHub token to toml file
    pub fn save_token(platform: &str, token: &str) -> std::io::Result<()> {
        let config = Config::new();

        let toml = toml::to_string(&Oauth::new(platform, token));

        match toml {
            Ok(contents) => {
                std::fs::write(config.path.as_path(), contents).unwrap();
            }
            Err(err) => println!("Failed to save token - [{}]", err),
        }
        Ok(())
    }
}
