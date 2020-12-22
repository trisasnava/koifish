use log::{error, info};
use structopt::StructOpt;

use crate::config::{Config, Oauth};
use crate::handler;
use crate::logger::Logger;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "
    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█  ")]
pub struct Koifish {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Subcommand
    #[structopt(flatten)]
    fish: Fish,
}

impl Koifish {
    pub fn run() {
        match Koifish::from_args() {
            Koifish { debug, fish } => {
                if debug {
                    match Logger::level("debug") {
                        Ok(logger) => info!("Log path [{}]", logger.log_path.display()),
                        Err(_e) => error!("Logger setup error!"),
                    };
                }
                Fish::run(fish);
            }
        }
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Fish {
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
        more: bool,
    },
}

impl Fish {
    fn run(self) {
        match self {
            Fish::Login { re_oauth } => {
                Self::login(re_oauth);
            }
            Fish::Join { channel } => {
                Self::join(channel);
            }
            Fish::Open { channel } => {
                Self::open(channel);
            }
            Fish::Meeting => {
                Self::meeting();
            }
            Fish::Upgrade { re_oauth, more } => {
                Self::upgrade(re_oauth, more).unwrap();
            }
        }
    }

    /// login to GitHub
    fn login(re_oauth: bool) {
        println!("Signing in with GitHub Oauth...");

        if re_oauth {
            handler::oauth::oauth();
        }

        match Config::new().read() {
            Ok(config) => match config.oauth {
                Oauth { token, .. } => match token {
                    Some(token) => {
                        if token.is_empty() {
                            handler::oauth::oauth();
                        }
                        handler::cli::login(token.as_str()).unwrap();
                    }
                    None => handler::oauth::oauth(),
                },
            },
            Err(err) => error!("Failed to read configuration file - {}", err),
        }
    }

    /// join slack channel
    fn join(channel: String) {
        handler::cli::join(channel);
    }

    /// Open Koifish site
    fn open(channel: String) {
        handler::cli::open(channel);
    }

    /// Start a meeting
    fn meeting() {
        handler::cli::meeting();
    }

    /// Upgrade koifish
    fn upgrade(re_oauth: bool, more: bool) -> std::io::Result<()> {
        println!("Koifish is upgrading...");

        if re_oauth {
            handler::oauth::oauth();
        }

        match Config::new().read() {
            Ok(config) => match config.oauth {
                Oauth { token, .. } => match token {
                    None => handler::oauth::oauth(),
                    Some(token) => {
                        if token.is_empty() {
                            handler::oauth::oauth();
                        }
                        handler::cli::login(token.as_str()).unwrap();
                        handler::cli::upgrade(token.as_str(), more);
                    }
                },
            },
            Err(err) => error!("Failed to read configuration file - {}", err),
        }
        Ok(())
    }
}
