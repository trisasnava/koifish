use super::widgets::details::Details;
use super::widgets::home::Home;
use super::widgets::search::Search;

/// Koifish TUI
pub struct Koifish {
    home: Home,
    search: Search,
    details: Details,
}

/// Koifish TUI implement
impl Koifish {
    pub fn run() {
        Details::draw();
        Home::draw();
        Search::draw();
    }
}
