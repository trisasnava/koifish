use super::widgets::details::Details;
use super::widgets::home::Home;
use super::widgets::search::Search;

pub struct TUI {
    home: Home,
    search: Search,
    details: Details,
}

impl TUI {
    pub fn run() {
        Details::draw();
        Home::draw();
        Search::draw();
    }
}
