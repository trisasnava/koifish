extern crate dirs;

use std::path::Path;
use std::{env, fs};

use github_rs::client::{Executor, Github};
use log::error;
use log::info;
use serde_json::Value;
use webbrowser;

use crate::utils;
use crate::utils::network;
use crate::utils::Counter;

/// Print login info
pub fn login(token: &str) -> Result<(), reqwest::Error> {
    let client = Github::new(token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((_, _, json)) => {
            if let Some(json) = json {
                println!(
                    "Login successful!\nHi,{},I am Koi",
                    json["name"].as_str().unwrap()
                );
            }
        }
        Err(_) => {
            Counter::new(20)
                .count()
                .msg("Login failed, please check the network and try again.");
            login(token).unwrap();
        }
    }
    Ok(())
}

/// Open some link in browser for koifish CLI
pub fn open(channel: String) {
    const GITHUB: &str = "https://github.com/trisasnava";
    const WEBSITE: &str = "https://trisasnava.org";
    const DOCS: &str = "https://trisasnava.org/koifish";
    const SLACK: &str =
        "https://join.slack.com/t/trisasnava/shared_invite/zt-pzee45is-Xlqax8Oa3JC0T7rkqcr4xw";
    const DISCORD: &str = "https://discord.gg/FztbBXbq";

    match channel.as_str() {
        "github" => {
            if webbrowser::open(GITHUB).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        "site" => {
            if webbrowser::open(WEBSITE).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        "website" => {
            if webbrowser::open(WEBSITE).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        "docs" => {
            if webbrowser::open(DOCS).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        "slack" => {
            if webbrowser::open(SLACK).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        "discord" => {
            if webbrowser::open(DISCORD).is_err() {
                error!("Open {:?} failure !", channel);
            }
        }
        _ => {
            error!("No such channel '{}'", channel);
        }
    }
}

/// Start a online meeting
pub fn meeting() {
    const MEET: &str = "https://meet.jit.si/koi";

    if webbrowser::open(MEET).is_err() {
        error!("Open Meeting failure !");
    }
}

/// Update
pub fn update(token: &str, verbose: bool) {
    let client = Github::new(token).unwrap();
    let release = client
        .get()
        .custom_endpoint("repos/trisasnava/koifish/releases/latest")
        .execute::<Value>();

    match release {
        Ok((_headers, _, json_value)) => {
            if let Some(latest) = json_value {
                if latest["assets"].is_array() {
                    let list = latest["assets"].as_array().unwrap();
                    for release in list.iter() {
                        // is latest
                        match release["name"].as_str() {
                            Some(os) => {
                                if os.contains(std::env::consts::OS) {
                                    if verbose {
                                        print_release(&latest, &release);
                                    }

                                    // download bin file
                                    let download_url =
                                        release["browser_download_url"].as_str().unwrap();
                                    let tmp_file = Path::new(dirs::cache_dir().unwrap().as_path())
                                        .join(release["name"].as_str().unwrap());

                                    match network::download_form_github(
                                        download_url,
                                        &*tmp_file,
                                        true,
                                    ) {
                                        Ok(..) => {
                                            info!("Successfully downloaded the binary from github")
                                        }
                                        Err(err) => println!(
                                            "Failed to download binary from github - [{}]",
                                            err
                                        ),
                                    }

                                    let bak_file = Path::new(dirs::cache_dir().unwrap().as_path())
                                        .join("koi.bak");

                                    // replace old bin file
                                    match utils::self_replace(&*tmp_file, bak_file.as_path()) {
                                        Ok(..) => println!(
                                            "Update to v{}",
                                            &latest["tag_name"].as_str().unwrap()
                                        ),
                                        Err(e) => {
                                            fs::rename(
                                                bak_file,
                                                env::current_exe().unwrap().as_path(),
                                            )
                                            .unwrap();
                                            error!("Self replace  error - {}", e);
                                        }
                                    }
                                }
                            }
                            None => {
                                println!("No matching version!");
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Update error, retrying - [{}]", e);
            Counter::new(20)
                .count()
                .msg("Update failed, please check the network and try again.");
            update(token, verbose);
        }
    }
}

fn print_release(latest: &Value, release: &Value) {
    println!(
        "OS: \"{}({})\"",
        std::env::consts::OS,
        release["name"].as_str().unwrap()
    );
    println!("Build time: {}", latest["created_at"]);
    println!(
        "Release version: \"Koi {}\"",
        latest["tag_name"].as_str().unwrap()
    );
    println!("Release notes: \"\n{}\"", latest["body"].as_str().unwrap());
}
