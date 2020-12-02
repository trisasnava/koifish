#![feature(option_result_contains)]
extern crate dirs;

use std::{env, fs};
use std::fs::File;
use std::io::{copy, Write};
use std::path::Path;

use console::Emoji;
use github_rs::client::{Executor, Github};
use github_rs::headers::{etag, rate_limit_remaining};
use serde_json::Value;
use webbrowser;

use crate::utils;
use crate::utils::counter;
use crate::utils::network;

/// Echo GitHub user name for koifish CLI
pub fn echo_username(token: &str) -> Result<(), reqwest::Error> {
    let client = Github::new(token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((headers, _, json)) => {
            if let Some(etag) = etag(&headers) {
                client.get().set_etag(etag).user().execute::<Value>();
            }

            if let Some(json) = json {
                println!(
                    "Login successful - [Hi,{},I am {}]",
                    json["name"],
                    Emoji("🐠 ", "Koi")
                );
            }
        }
        Err(e) => unsafe {
            println!("Login error, retrying - [{}]", e);
            counter::num_counter(20, "Login failed, please check the network and try again.");
            echo_username(token);
        },
    }
    Ok(())
}

/// Open some link in browser for koifish CLI
pub fn open(channel: String) {
    let github = "https://github.com/trisasnava";
    let website = "https://trisasnava.org";
    let docs = "https://trisasnava.org/koifish";

    if channel.as_str() == "github" {
        if webbrowser::open(github).is_ok() {
            println!("Open {:?} successful !", channel);
        } else {
            println!("Open {:?} failure !", channel);
        };
    } else if channel.as_str() == "website" {
        if webbrowser::open(website).is_ok() {
            println!("Open {:?} successful !", channel);
        } else {
            println!("Open {:?} failure !", channel);
        };
    } else if channel.as_str() == "docs" {
        if webbrowser::open(docs).is_ok() {
            println!("Open {:?} successful !", channel);
        } else {
            println!("Open {:?} failure !", channel);
        };
    } else {
        println!("Open {:?} failure !", channel);
    }
}

/// Join koifish channel in koifish CLI
pub fn join() {
    let slack = "https://trisasnava.slack.com/join/shared_invite/enQtODg1NjI0NTc1NzAz\
    LTBjYTM1YjQxZWZkMTExYTBlNTcxNjQzYTc0MjRmNDNjMmIxZmMwZjM5ODFkZWExNjJkNWMwZWRjOGJlODdiM2Q";

    if webbrowser::open(slack).is_ok() {
        println!("Open slack successful !");
    } else {
        println!("Open Slack failure !");
    }
}

/// Start a meeting with koifish CLI
pub fn meet() {
    let meet = "https://meet.jit.si/koi";

    if webbrowser::open(meet).is_ok() {
        println!("Open Meet successful !");
    } else {
        println!("Open Meet failure !");
    }
}

/// Upgrade tool for koifish
pub fn upgrade(token: &str, verbose: bool) {
    let client = Github::new(token).unwrap();
    let release = client
        .get()
        .custom_endpoint("repos/trisasnava/koifish/releases/latest")
        .execute::<Value>();

    match release {
        Ok((headers, _, json_value)) => {
            if let Some(etag) = etag(&headers) {
                client
                    .get()
                    .set_etag(etag)
                    .custom_endpoint("repos/trisasnava/koifish/releases/latest")
                    .execute::<Value>();
            }

            if let Some(latest) = json_value {
                if latest["assets"].is_array() {
                    let list = latest["assets"].as_array().unwrap();
                    for release in list.iter() {
                        // is latest
                        match release["name"].as_str() {
                            Some(os) => {
                                if os.contains(std::env::consts::OS) {
                                    if verbose {
                                        echo_release(&latest, &release);
                                    }

                                    // download bin file
                                    let download_url =
                                        release["browser_download_url"].as_str().unwrap();
                                    let tmp_file = Path::new(dirs::cache_dir().unwrap().as_path())
                                        .join(release["name"].as_str().unwrap());
                                    network::download_form_github(download_url, &*tmp_file);

                                    let bak_file = Path::new(dirs::cache_dir().unwrap().as_path())
                                        .join("koi.bak");

                                    // replace old bin file
                                    match utils::self_replace(&*tmp_file, bak_file.as_path()) {
                                        Ok(..) => println!(
                                            "Upgrade to {}",
                                            &latest["tag_name"].as_str().unwrap()
                                        ),
                                        Err(e) => {
                                            fs::rename(
                                                bak_file,
                                                env::current_exe().unwrap().as_path(),
                                            );
                                        }
                                    }
                                }
                            }
                            None => {
                                println!("No corresponding version found!");
                            }
                        }
                    }
                }
            }
        }
        Err(e) => unsafe {
            println!("Upgrade error, retrying - [{}]", e);
            counter::num_counter(
                20,
                "Upgrade failed, please check the network and try again.",
            );
            upgrade(token, verbose);
        },
    }
}

fn echo_release(latest: &Value, release: &Value) {
    println!(
        "OS: \"{}({})\"",
        std::env::consts::OS,
        release["name"].as_str().unwrap()
    );
    println!("Date: {}", latest["created_at"]);
    println!(
        "Release version: \"Koi {}\"",
        latest["tag_name"].as_str().unwrap()
    );
    println!("Release notes: \"\n{}\"", latest["body"].as_str().unwrap());
}