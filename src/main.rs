mod cli;
mod config;
mod handler;
mod logger;
mod tui;
mod utils;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() > 1 {
        cli::Koifish::run();
    } else {
        tui::draw().unwrap();
    }
}
