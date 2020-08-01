use console::Emoji;
use github_rs::client::{Executor, Github};
use serde_json::Value;

pub fn echo_username(token: &str) {
    let client = Github::new(token).unwrap();
    let me = client.get().user().execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
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
