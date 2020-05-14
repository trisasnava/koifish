use clap::{App, SubCommand};

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("
    █▄▀ █▀█ ░ █▀▀ ░ █▀ █░█
    █░█ █▄█ █ █▀▀ █ ▄█ █▀█")
        .version(" v0.0.1")
        .args_from_usage(
            "-s,  --start    'Start a web Koifish in local'
                -o,  --online   'Run a online Koifish in https://webassembly.sh'
                -j,  --join     'Join our Slack Channel or WeChat Group(QR-Code)'"
        )
        .subcommand(SubCommand::with_name("issue").about("Get GitHub issues info"))
        .subcommand(SubCommand::with_name("login").about("Login to GitHub"))
        .subcommand(SubCommand::with_name("pr").about("Get GitHub prs info"))
        .subcommand(SubCommand::with_name("repo").about("Get GitHub repo info"))
        .subcommand(SubCommand::with_name("trending").about("Get GitHub trending info"))
        .subcommand(SubCommand::with_name("user").about("Get GitHub user info"))
        .get_matches()
}

fn main() {
    get_matches();
}