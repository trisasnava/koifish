use std::io::Error;

use super::widgets::cli::Koifish;
use super::widgets::details::Details;
use super::widgets::home::Home;
use super::widgets::search::Search;

pub struct App {
    cli: Koifish,
    home: Home,
    search: Search,
    details: Details,
}

impl App {
    pub fn new_tui() {
        Home::draw();
        Search::draw();
        Details::draw();
    }

    pub fn new_cli() {
        Koifish::match_args();
    }
}