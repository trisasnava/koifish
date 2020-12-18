use structopt::StructOpt;

use crate::handler::cli;
use crate::handler::oauth;
use crate::model::conf::{Config, Oauth};

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "
    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█  ")]
pub enum KoiFish {
    /// Verify login via GitHub Oauth
    Login {
        /// Re-oauth via GitHub Oauth
        #[structopt(short, long)]
        re_oauth: bool,
    },
    /// Join our Slack/Discord channel,default is Slack
    Join {
        /// Join our Slack/Discord channel
        #[structopt(default_value = "slack")]
        channel: String,
    },
    /// Open Koifish GitHub/Website/Docs
    Open {
        #[structopt(default_value = "docs")]
        channel: String,
    },
    /// Start a meeting  at https://meet.jit.si/koig
    Meeting,
    /// Upgrade Koifish from GitHub Release
    Upgrade {
        /// Re-oauth via GitHub Oauth
        #[structopt(short, long)]
        re_oauth: bool,

        /// Release notes verbose output
        #[structopt(short, long)]
        verbose: bool,
    },
}

impl KoiFish {
    pub fn run() {
        // Self::print_matches();
        match KoiFish::from_args() {
            Self::Login { re_oauth } => {
                Self::login(re_oauth);
            }
            Self::Join { channel } => {
                Self::join(channel);
            }
            Self::Open { channel } => {
                Self::open(channel);
            }
            Self::Meeting => {
                Self::meeting();
            }
            Self::Upgrade { re_oauth, verbose } => {
                Self::upgrade(re_oauth, verbose);
            }
        }
    }

    /// login to GitHub
    fn login(re_oauth: bool) -> std::io::Result<()> {
        println!("Oauth logging...");

        if re_oauth {
            oauth::oauth();
        }

        match Config::new().read() {
            Ok(config) => match config.oauth {
                Oauth { token, .. } => match token {
                    Some(token) => {
                        if token.is_empty() {
                            oauth::oauth();
                        }
                        cli::echo_username(token.as_str());
                    }
                    None => oauth::oauth(),
                },
            },
            Err(err) => {
                println!("Failed to read configuration file - {}", err)
            }
        }

        Ok(())
    }

    /// join slack channel
    fn join(channel: String) {
        cli::join(channel);
    }

    /// Open Koifish site
    fn open(channel: String) {
        cli::open(channel);
    }

    /// Start a meeting
    fn meeting() {
        cli::meeting();
    }

    /// Upgrade koifish
    fn upgrade(re_oauth: bool, verbose: bool) -> std::io::Result<()> {
        println!("Koifish is upgrading...");

        if re_oauth {
            oauth::oauth();
        }

        match Config::new().read() {
            Ok(config) => match config.oauth {
                Oauth { token, .. } => match token {
                    None => oauth::oauth(),
                    Some(token) => {
                        if token.is_empty() {
                            oauth::oauth();
                        }
                        cli::echo_username(token.as_str());
                        cli::upgrade(token.as_str(), verbose);
                    }
                },
            },
            Err(err) => {
                println!("Failed to read configuration file - {}", err)
            }
        }

        Ok(())
    }
}
