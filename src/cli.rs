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
    /// Log in via GitHub Oauth
    Login {
        /// Re-oauth via GitHub Oauth
        #[structopt(short, long)]
        re_oauth: bool,
    },
    /// Open channels "docs,github,site,slack,discord"
    Open {
        /// Open channels "docs,github,site,slack,discord"
        #[structopt(default_value = "docs")]
        channel: String,
    },
    /// Start a online meeting via https://meet.jit.si/
    Meet,
    /// Update from GitHub release
    Update {
        /// Re-oauth via GitHub Oauth
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
            Fish::Open { channel } => {
                Self::open(channel);
            }
            Fish::Meet => {
                Self::meeting();
            }
            Fish::Update { re_oauth, verbose } => {
                Self::update(re_oauth, verbose).unwrap();
            }
        }
    }

    fn login(re_oauth: bool) {
        println!("Login via GitHub Oauth...");

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

    /// Open koifish channel
    fn open(channel: String) {
        handler::cli::open(channel);
    }

    /// Start a online meeting
    fn meeting() {
        handler::cli::meeting();
    }

    /// Update koifish CLI
    fn update(re_oauth: bool, verbose: bool) -> std::io::Result<()> {
        println!("Koifish is updating...");

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
                        handler::cli::update(token.as_str(), verbose);
                    }
                },
            },
            Err(err) => error!("Failed to read configuration file - {}", err),
        }
        Ok(())
    }
}
