use super::widgets::home::Home;

/// Koifish TUI
pub struct Koifish {
    home: Home,
}

/// Koifish TUI implement
impl Koifish {
    pub fn run() {
        Home::draw();
    }
}
