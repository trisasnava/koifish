mod tui;
mod handler;
mod model;
mod utils;
mod widgets;
mod cli;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() > 1 {
        cli::Koi::run();
    } else {
        tui::Koifish::run();
    }
}
