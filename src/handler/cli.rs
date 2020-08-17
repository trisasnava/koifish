use console::Emoji;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use webbrowser;

pub fn echo_username(token: &str) {
    let client = Github::new(token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((_, _, json)) => {
            if let Some(json) = json {
                println!("Hi,{},I am {}.", json["name"], Emoji("🐠 ", "Koi"));
            }
        }
        Err(e) => {
            println!("Login Error\n{}", e);
            println!("Retrying ...");
            echo_username(token)
        }
    }
}

pub fn open(channel: String) {
    let github = "https://github.com/trisasnava";
    let website = "https://trisasnava.org";
    let docs = "https://trisasnava.org/koifish";

    if channel.as_str() == "github" {
        if webbrowser::open(github).is_ok() {
            println!("Open {:?} successful !", channel);
        };
    } else if channel.as_str() == "website" {
        if webbrowser::open(website).is_ok() {
            println!("Open {:?} successful !", channel);
        };
    } else if channel.as_str() == "docs" {
        if webbrowser::open(docs).is_ok() {
            println!("Open {:?} successful !", channel);
        };
    } else {
        println!("Open {:?} failure !", channel);
    }
}

pub fn join() {
    let slack = "https://trisasnava.slack.com/join/shared_invite/enQtODg1NjI0NTc1NzAz\
    LTBjYTM1YjQxZWZkMTExYTBlNTcxNjQzYTc0MjRmNDNjMmIxZmMwZjM5ODFkZWExNjJkNWMwZWRjOGJlODdiM2Q";

    if webbrowser::open(slack).is_ok() {
        println!("Open slack successful !");
    } else {
        println!("Open Slack failure !");
    }
}

pub fn meet() {
    let meet = "https://meet.jit.si/koi";

    if webbrowser::open(meet).is_ok() {
        println!("Open Meet successful !");
    } else {
        println!("Open Meet failure !");
    }
}

pub fn upgrade() {}
