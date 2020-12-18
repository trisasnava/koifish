mod cli;
mod handler;
mod model;
mod tui;
mod utils;
mod widgets;

#[paw::main]
fn main(args: paw::Args) {
    use chrono;

    fn main() {
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
    }

    if args.len() > 1 {
        cli::KoiFish::run();
    } else {
        tui::Koifish::run();
    }
}
