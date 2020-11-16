extern crate dirs;

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use reqwest;
use reqwest::Url;
use serde_json;
use webbrowser;

use crate::handler::cli;
use crate::model::oauth::OauthToken;

/// GitHub OAuth for koifish
pub fn oauth() {
    let login_url = "https://koifish.trisasnava.org";
    let address = "localhost:3690";

    if webbrowser::open(login_url).is_ok() {
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
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
                    oauth_save_token(value.to_string());
                    response(stream);
                    break;
                }
                Err(_e) => println!("Verification failed, please try again."),
            }
        }
    }
}

/// Get Github token and save token to toml file
#[tokio::main]
async fn oauth_save_token(code: String) -> Result<(), reqwest::Error> {
    let oauth_url = "https://koifish.trisasnava.org";

    let res: serde_json::Value = reqwest::Client::new()
        .post(oauth_url)
        .json(&serde_json::json!({ "code": code }))
        .send()
        .await?
        .json()
        .await?;

    match res["token"].as_str() {
        Some(token) => {
            cli::echo_username(res["token"].to_string().replace("\"", "").as_str());
            sava_token(OauthToken::new(token));
        }
        None => println!("Token acquisition failed, please try again."),
    }
    Ok(())
}

/// Save GitHub token to toml file
fn sava_token(token: OauthToken) {
    let token_contents = format!("[oauth_token]\ntoken=\"{}\"", token.value().to_string());

    let config = Path::new(dirs::home_dir().unwrap().as_path()).join(".koi");
    fs::write(config, token_contents).expect("Could not write to file!");
}

/// Response with static file
fn response(mut stream: TcpStream) {
    let index_html = "<!DOCTYPE html><html><head><meta name=\"viewport\" content=\"initial-scale=1, maximum-scale=1, \
    user-scalable=no\"/><title>Oauth Login</title><style type=\"text/css\">html, body {overflow: hidden; margin: 0;background: #000}\
    body{font-family: 'Open Sans', 'Helvetica Neue', 'Hiragino Sans GB', 'LiHei Pro', Arial, sans-serif;color: #333}\
    #wrapper {position: absolute;width: 320px;text-align: center;top: 50%;left: 50%;margin-left: -160px;margin-top: -160px;\
    -webkit-user-select: none;-moz-user-select: none;user-select: none}h1 {font-family: 'Montserrat', 'Helvetica Neue', Arial,\
    sans-serif;font-weight: 700;font-size: 20px;letter-spacing: 3px;text-transform: uppercase;color: #eee;margin: 40px 0;\
    position: relative}p, input {font-size: 14px;line-height: 2em;margin: 0;letter-spacing: 2px}input { width: 140px;\
    line-height:38px;text-align: center;font-weight: bold;color: #fff;text-shadow: 1px 1px 1px #333;border-radius: 5px;\
    margin: 0 20px 20px 0;position: relative;overflow: hidden;border: none;outline: none;}.inline {width: 60px; float: left;\
    display: inline}.ant-btn {line-height: 1.499;position: relative;display: inline-block;font-weight: 400;white-space: \
    nowrap;text-align: center;background-image: none;border: 1px solid transparent;-webkit-box-shadow: 0 2px 0 rgba(0, 0, \
    0, 0.015);box-shadow: 0 2px 0 rgba(0, 0, 0, 0.015);cursor: pointer;-webkit-transition: all .3s cubic-bezier(.645, .045, \
    .355, 1);transition: all .3s cubic-bezier(.645, .045, .355, 1);-webkit-user-select: none;-moz-user-select: none;\
    -ms-user-select: none;user-select: none;-ms-touch-action: manipulation;touch-action: manipulation;height: 32px;padding: \
    0 15px;font-size: 14px;border-radius: 4px;color: rgba(0, 0, 0, 0.65);background-color: #fff;border-color: #d9d9d9;}\
    .ant-btn-red {color: #fff;background-color: #000;border-color: #FF5A44;text-shadow: 0 -1px 0 rgba(0, 0, 0, 0.12);\
    -webkit-box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);box-shadow: 0 2px 0 rgba(0, 0, 0, 0.045);}</style></head><body>\
    <script language=\"javascript\">function custom_close(){    window.close();}</script><div id=\"wrapper\">
    <h1>&#128032;</h1><h1>Login successful!</h1><div class=\"inline\"><input class=\"\
    ant-btn ant-btn-red\" type=\"button\" value=\"< Back to CLI\" onClick=\"custom_close()\"/></div><div style=\" \
    float:right;\"><input class=\"ant-btn ant-btn-red\" type=\"button\" value=\"How to use >\"onClick=\"\
    window.location.href='https://trisasnava.org/koifish'\"/></div></div></body></html>";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", index_html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}