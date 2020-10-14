use std::fs::File;
use std::io::Read;
use std::path::Path;

use structopt::StructOpt;
use toml;

use crate::handler::cli;
use crate::handler::oauth;
use crate::model::conf::Config;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "
    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█  ")]
pub enum Koi {
    /// Verify login via GitHub Oauth
    Login,
    /// Join our slack channel
    Join,
    /// Open koifish github|website|docs
    Open {
        #[structopt(default_value = "docs")]
        channel: String,
    },
    /// Start a meeting with https://meet.jit.si/koi
    Meet,
    /// Upgrade tool for Koifish
    Upgrade,
    // /// Get GitHub user info.
    // User {
    //     #[structopt(default_value = "trisasnava")]
    //     user_or_org: String,
    // },
    // /// Get GitHub repo info.
    // Repo {
    //     #[structopt(default_value = "trisasnava")]
    //     user_or_org: String,
    //     #[structopt(default_value = "koifish")]
    //     repo: String,
    // },
    // /// Get GitHub issues info in your repo.
    // Issues {
    //     #[structopt(default_value = "trisasnava")]
    //     user_or_org: String,
    //     #[structopt(default_value = "koifish")]
    //     repo: String,
    // },
    // /// Get GitHub prs info for your repo.
    // Prs {
    //     #[structopt(default_value = "trisasnava")]
    //     user_or_org: String,
    //     #[structopt(default_value = "koifish")]
    //     repo: String,
    // },
    // /// Get GitHub trending repo info.
    // #[structopt(help = "Fitter by daily|weekly|monthly,The default is daily.")]
    // Trending {
    //     #[structopt(default_value = "daily")]
    //     date: String,
    // },
}

impl Koi {
    /// Match Options
    pub fn run() {
        // Self::print_matches();
        match Koi::from_args() {
            Self::Login => {
                Self::login();
            }
            Self::Join => {
                Self::join();
            }
            Self::Open { channel } => {
                Self::open(channel);
            }
            Self::Meet => {
                Self::meet();
            }
            Self::Upgrade => {
                Self::upgrade();
            }
            _ => {}
        }
    }

    /// print matches for test
    fn print_matches() {
        println!("{:#?}", Self::from_args());
    }

    /// login to GitHub
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
                                    cli::echo_username(token.get_token());
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

    /// join slack channel
    fn join() {
        cli::join();
    }

    /// Open Koifish site
    fn open(channel: String) {
        cli::open(channel);
    }

    /// Start a meeting
    fn meet() {
        cli::meet();
    }

    /// Upgrade koifish
    fn upgrade() -> std::io::Result<()> {
        match dirs::home_dir() {
            Some(home) => {
                let config = Path::new(home.as_path()).join(".koi");
                match config.exists() {
                    false => {
                        oauth::oauth();
                        let mut file = File::open(config.as_path())?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;

                        let config: Config = toml::from_str(contents.as_str()).unwrap();

                        match config {
                            token => {
                                if token.get_token().len() > 0 {
                                    cli::upgrade(token.get_token());
                                }
                            }
                        }
                    }
                    true => {
                        let mut file = File::open(config.as_path())?;
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;

                        let config: Config = toml::from_str(contents.as_str()).unwrap();

                        match config {
                            token => {
                                if token.get_token().len() > 0 {
                                    cli::upgrade(token.get_token());
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
}
