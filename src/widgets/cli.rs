use std::fs::File;
use std::io::Read;
use std::path::Path;

use structopt::StructOpt;
use toml;

use crate::handler::join;
use crate::handler::login;
use crate::handler::oauth;
use crate::model::conf::Config;
use crate::model::oauth::OauthToken;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "
    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█  ")]
pub enum Koifish {
    /// Run a online Koifish in https://webassembly.sh
    Online,
    /// Join our slack | github | website
    Join {
        #[structopt(default_value = "slack")]
        channel: String,
    },
    /// Start a web Koifish in local with your port.
    Web {
        #[structopt(default_value = "2121")]
        port: String,
    },
    /// Login to GitHub.
    Login,
    /// Get GitHub user info.
    User {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
    },
    /// Get GitHub repo info.
    Repo {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub issues info in your repo.
    Issues {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub prs info for your repo.
    Prs {
        #[structopt(default_value = "trisasnava")]
        user_or_org: String,
        #[structopt(default_value = "koifish")]
        repo: String,
    },
    /// Get GitHub trending repo info.
    #[structopt(help = "Fitter by daily|weekly|monthly,The default is daily.")]
    Trending {
        #[structopt(default_value = "daily")]
        date: String,
    },
}

impl Koifish {
    /// Match Options
    pub fn run() {
        // Self::print_matches();
        match Koifish::from_args() {
            Koifish::Login => {
                Self::login();
            }
            Koifish::Join { channel } => {
                Self::join(channel);
            }
            _ => {}
        }
    }

    /// print matches for test
    fn print_matches() {
        println!("{:#?}", Koifish::from_args());
    }

    // login to GitHub
    fn login() -> std::io::Result<()> {
        match dirs::home_dir() {
            Some(home) => {
                let config = Path::new(home.as_path()).join(".koi");
                match config.exists() {
                    false => {
                        oauth::oauth();
                    }
                    true => {
                        let mut file = File::open(config.as_path())?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;

                        let config: Config = toml::from_str(contents.as_str()).unwrap();

                        match config {
                            token => {
                                if token.get_token().len() > 0 {
                                    login::echo_username(token.get_token());
                                } else {
                                    oauth::oauth();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    // join slack channel
    fn join(channel: String) {
        join::join(channel);
    }
}
