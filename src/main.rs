mod cli;
mod handler;
mod model;
mod tui;
mod utils;
mod widgets;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() > 1 {
        cli::Koi::run();
    } else {
        tui::Koifish::run();
    }
}
