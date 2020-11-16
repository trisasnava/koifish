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
    /// verify login via GitHub Oauth
    Login {
        /// re-oauth with GitHub
        #[structopt(short, long)]
        re_oauth: bool,
    },
    /// join our slack channel
    Join,
    /// open koifish github|website|docs
    Open {
        #[structopt(default_value = "docs")]
        channel: String,
    },
    /// start a meeting with https://meet.jit.si/koi
    Meet,
    /// upgrade tool for Koifish
    Upgrade {
        /// re-oauth with GitHub
        #[structopt(short, long)]
        re_oauth: bool,

        /// release notes verbose output
        #[structopt(short, long)]
        verbose: bool,
    },
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
    pub fn run() {
        // Self::print_matches();
        match Koi::from_args() {
            Self::Login { re_oauth } => {
                Self::login(re_oauth);
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
            Self::Upgrade { re_oauth, verbose } => {
                Self::upgrade(re_oauth, verbose);
            }
            _ => {}
        }
    }

    /// print matches for test
    fn print_matches() {
        println!("{:#?}", Self::from_args());
    }

    /// login to GitHub
    fn login(re_oauth: bool) -> std::io::Result<()> {
        println!("Oauth logging in...");

        if re_oauth {
            oauth::oauth();
        }

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
    fn upgrade(re_oauth: bool, verbose: bool) -> std::io::Result<()> {
        println!("Koifish is upgrading...");

        if re_oauth {
            oauth::oauth();
        }

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
                                    cli::upgrade(token.get_token(), verbose);
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
                                    cli::upgrade(token.get_token(), verbose);
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
