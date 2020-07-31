use app::TUI;
use widgets::cli::Koifish;

mod app;
mod widgets;
mod model;
mod handler;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() > 1 {
        Koifish::run();
    } else {
        TUI::run();
    }
}