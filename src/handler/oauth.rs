extern crate dirs;

use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener};

use log::{error, info};
use reqwest;
use reqwest::Url;
use serde_json;
use webbrowser;

use crate::config::Config;

/// GitHub OAuth for koifish
pub fn oauth() {
    let oauth_url = "https://koi.trisasnava.org";
    let addrs = SocketAddr::from(([127, 0, 0, 1], 3690));

    if webbrowser::open(&oauth_url).is_err() {
        oauth();
    }

    let listener = TcpListener::bind(addrs).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(&stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                match oauth_and_save_token(&oauth_url, value.to_string()) {
                    Ok(..) => {
                        let html_contents = include_str!("../static/index.html");
                        let response_data =
                            format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", html_contents);
                        stream.write(response_data.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        info!("GitHub Oauth successful!");
                    }
                    Err(err) => error!("GitHub Oauth Error {}", err),
                }
                break;
            }
            Err(err) => println!("Verification failed, please try again. {}", err),
        }
    }
}

/// Get Github token and save token to toml file
#[tokio::main]
async fn oauth_and_save_token(oauth_url: &str, code: String) -> Result<(), reqwest::Error> {
    let res: serde_json::Value = reqwest::Client::new()
        .post(oauth_url)
        .json(&serde_json::json!({ "code": code }))
        .send()
        .await?
        .json()
        .await?;

    match res["token"].as_str() {
        Some(token) => {
            Config::save_token("GitHub", &token).unwrap();
            info!("Token saved.")
        }
        None => println!("Token acquisition failed, please try again."),
    }
    Ok(())
}
