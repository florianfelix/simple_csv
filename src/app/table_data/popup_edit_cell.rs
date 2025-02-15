use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default, Setters)]
pub struct PopupEditCell<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
    #[setters(into)]
    title_bottom: Line<'a>,
}

impl Widget for PopupEditCell<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .title_bottom(self.title_bottom)
            .borders(Borders::ALL)
            .border_style(self.border_style);

        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
