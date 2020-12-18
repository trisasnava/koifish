use std::io;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub struct Home {
    user_info: Layout,
    repo_info: Layout,
    trending_repo: Layout,
    trending_developer: Layout,
    issue_info: Layout,
    assigned_issue: Layout,
}

impl Home {
    pub fn draw() -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|mut f| {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(f.size().height as u16),
                        Constraint::Length(100),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Min(5),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            const REQUIRED_COLUMNS: usize = 4;

            let chunk_vec =
                vec![Constraint::Percentage((100 / REQUIRED_COLUMNS) as u16); REQUIRED_COLUMNS];
            let chunks = Layout::default()
                .constraints(chunk_vec.as_ref())
                .direction(Direction::Horizontal)
                .split(vertical_chunks[0]);

            let block = Block::default().title("User Info").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let block = Block::default().title("Repo Info").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);

            let block = Block::default().title("Issues Info").borders(Borders::ALL);
            f.render_widget(block, chunks[2]);

            let block = Block::default()
                .title("Assigned Info")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[3]);
        })
    }
}
