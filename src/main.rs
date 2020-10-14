use app::Koifish;
use widgets::cli::Koi;

mod app;
mod handler;
mod model;
mod utils;
mod widgets;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() > 1 {
        Koi::run();
    } else {
        Koifish::run();
    }
}
