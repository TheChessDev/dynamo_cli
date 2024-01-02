use std::io::stdout;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(f.size());

        let block = Block::default()
            .title("DynamoDB Tables")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
    })?;

    Ok(())
}
