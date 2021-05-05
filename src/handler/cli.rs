extern crate dirs;

use std::path::{Path, PathBuf};

use github_rs::client::{Executor, Github};
use log::error;
use log::info;
use serde_json::Value;
use webbrowser;

use crate::utils;
use crate::utils::network;
use crate::utils::Counter;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::io::Write;
use std::{env, fs, io};
use url::Url;

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

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

// TODO replace by Progress Bar
fn progress(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

/// Speed up git clone via `https://github.com.cnpmjs.org`
pub fn clone(url: String, path: String) {
    match Url::parse(url.as_str()) {
        Ok(url) => {
            let new_url = format!("https://github.com.cnpmjs.org{}", url.path());

            let project_name: Vec<_> = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap()
                [1]
            .split(".")
            .collect();
            let new_path;
            if path.as_str() == "." {
                new_path = env::current_dir().unwrap().as_path().join(project_name[0]);
            } else {
                new_path = Path::new(&path).join(project_name[0]);
            }

            let state = RefCell::new(State {
                progress: None,
                total: 0,
                current: 0,
                path: None,
                newline: false,
            });
            let mut cb = RemoteCallbacks::new();
            cb.transfer_progress(|stats| {
                let mut state = state.borrow_mut();
                state.progress = Some(stats.to_owned());
                progress(&mut *state);
                true
            });

            let mut co = CheckoutBuilder::new();
            co.progress(|path, cur, total| {
                let mut state = state.borrow_mut();
                state.path = path.map(|p| p.to_path_buf());
                state.current = cur;
                state.total = total;
                progress(&mut *state);
            });

            let mut fo = FetchOptions::new();
            fo.remote_callbacks(cb);
            RepoBuilder::new()
                .fetch_options(fo)
                .with_checkout(co)
                .clone(new_url.as_str(), new_path.as_path())
                .unwrap();
            println!();
        }
        Err(_) => {
            panic!("Repo url is not available");
        }
    };
}

/// Open some link in browser for koifish CLI
pub fn open(channel: String) {
    const GITHUB: &str = "https://github.com/trisasnava";
    const WEBSITE: &str = "https://trisasnava.org";
    const DOCS: &str = "https://trisasnava.org/koifish";
    const SLACK: &str =
        "https://join.slack.com/t/trisasnava/shared_invite/zt-pzee45is-Xlqax8Oa3JC0T7rkqcr4xw";
    const DISCORD: &str = "https://discord.gg/pjbF6Hwez9";

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
