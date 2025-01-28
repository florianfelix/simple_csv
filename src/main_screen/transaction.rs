use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub name: String,
    pub amount: f32,
}
impl Default for Transaction {
    fn default() -> Self {
        Self {
            name: String::from("Transaction Name"),
            amount: 10.50,
        }
    }
}
impl Widget for Transaction {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.left(),
            area.top(),
            &self.name,
            Style::default().fg(Color::Green),
        );
    }
}
impl Transaction {
    pub fn render_self(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Current Transaction");

        let lines = vec![
            Line::from(self.name.clone()),
            Line::from(self.amount.to_string()),
        ];
        let p = Paragraph::new(lines).block(block);

        frame.render_widget(p, area);
    }
}
