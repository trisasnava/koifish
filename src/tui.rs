use std::io;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::Terminal;
use tui::widgets::{Block, Borders, BorderType};

pub fn draw() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let block = Block::default()
                .borders(Borders::ALL)
                .title("[ Koifish ]")
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(BorderType::Double);
            f.render_widget(block, f.size());

            let back_chunks = Layout::default()
                .horizontal_margin(4)
                .vertical_margin(1)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let card_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(back_chunks[0]);

            let block = Block::default()
                .title("[ Commits info ]")
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .border_type(BorderType::Double);
            f.render_widget(block, card_chunks[0]);

            let block = Block::default()
                .title("[ Prs info ]")
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                .border_type(BorderType::Double);

            f.render_widget(block, card_chunks[1]);
        })?;
    }
}
