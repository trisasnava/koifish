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
        /// Re-oauth via github Oauth
        #[structopt(short, long)]
        re_oauth: bool,
    },
    /// Join our slack/discord channel,default is slack
    Join {
        /// Join our slack/discord channel
        #[structopt(default_value = "slack")]
        channel: String,
    },
    /// Open Koifish github/site/docs
    Open {
        #[structopt(default_value = "docs")]
        channel: String,
    },
    /// Start a meeting  at https://meet.jit.si/koig
    Meet,
    /// Upgrade Koifish from github release
    Upgrade {
        /// Re-oauth via github Oauth
        #[structopt(short, long)]
        re_oauth: bool,

        /// Release notes verbose output
        #[structopt(short, long)]
        verbose: bool,
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
            Fish::Meet => {
                Self::meeting();
            }
            Fish::Upgrade { re_oauth, verbose } => {
                Self::upgrade(re_oauth, verbose).unwrap();
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
    fn upgrade(re_oauth: bool, verbose: bool) -> std::io::Result<()> {
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
                        handler::cli::upgrade(token.as_str(), verbose);
                    }
                },
            },
            Err(err) => error!("Failed to read configuration file - {}", err),
        }
        Ok(())
    }
}
