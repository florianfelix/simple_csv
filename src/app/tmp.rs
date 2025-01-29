use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};

use super::App;

pub fn original(app: &mut App) -> Paragraph<'_> {
    let r = Paragraph::new(format!(
        "This is a tui template.\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press left and right to increment and decrement the counter respectively.\n\
            Counter: {}",
        app.counter
    ))
    .block(
        Block::bordered()
            .title("Template")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    .centered();
    r
}
