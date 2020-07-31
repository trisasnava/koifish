use std::{env, io, thread};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use oauth2::url::Url;
use reqwest;
use serde_json;
use webbrowser;

use crate::model::oauth::OauthToken;

pub fn oauth() {
    let login_url = "https://koifish.trisasnava.org/login";
    let address = "localhost:3690";

    if webbrowser::open(login_url).is_ok() {
        let listener = TcpListener::bind(address).unwrap();
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
                    oauth_save_token(value.to_string());
                    handle_client(stream);
                    break;
                }
                Err(e) => {
                    println!("Loin failure!")
                }
            }
        }
    }
}

#[tokio::main]
async fn oauth_save_token(code: String) -> Result<(), reqwest::Error> {
    let oauth_url = "https://koifish.trisasnava.org/oauth";
    let mut token;

    let res: serde_json::Value = reqwest::Client::new()
        .post(oauth_url)
        .json(&serde_json::json!({"code":code}))
        .send()
        .await?
        .json()
        .await?;

    token = OauthToken::new(res["token"].to_string());
    println!("{:?}", token.value());

    //TODO Save token to user profile
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let index_html = "<head><meta name=\"viewport\" content=\"initial-scale=1, maximum-scale=1, user-scalable=no\"/>
    <title>Login</title> <style type=\"text/css\">html, body {overflow: hidden;margin: 0;background: #000}
    body {font-family: 'Open Sans', 'Helvetica Neue', 'Hiragino Sans GB', 'LiHei Pro', Arial, sans-serif;color: #333}
    #wrapper{position: absolute;left: 0;width: 320px;text-align: center;top: 50%;left: 50%;margin-left: -160px;margin-top:
    -160px;-webkit-user-select: none;-moz-user-select: none;user-select: none} h1 {font-family: 'Montserrat', 'Helvetica Neue',
    Arial, sans-serif;font-weight: 700;font-size: 30px;letter-spacing: 9px;text-transform: uppercase;color: #eee;margin: 12px 0;
    left: 4px} h2 {color: #999;font-weight: normal;font-size: 15px;letter-spacing: .12em;margin-bottom: 30px;left: 3px}
    h1, h2 {position: relative} input {font-size: 14px;line-height: 2em;margin: 0;letter-spacing: 2px}
    canvas {position: absolute;top: 0;left: 0;z-index: 0; width: 100%; height: 100%; pointer-events: none}
    a {color: #999; text-decoration: none; transition: color .2s ease} a:hover {color: #f33}
    </style> </head> <body> <script language=\"javascript\">function custom_close(){ window.close();}</script><div id=\"wrapper\">
    <h1>Login successfully!</h1><input align=\"center\" id=\"btnClose\"type=\"button\" value=\"&lt; Back to Terminal :)\" \
    onClick=\"custom_close()\"/></body>";

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", index_html);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}