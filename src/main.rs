use app::App;

mod handler;
mod widgets;
mod model;
mod app;

#[paw::main]
fn main(args: paw::Args) {
    if args.len() == 1 {
        App::new_tui();
    } else {
        App::new_cli();
    }
}