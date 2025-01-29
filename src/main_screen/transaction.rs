use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table, Widget},
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
    pub fn render_as_table(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Current Transaction");

        let widths = [Constraint::Percentage(15), Constraint::Percentage(85)];

        let table = Table::new(self.as_rows(), widths).block(block);

        frame.render_widget(table, area);
    }

    fn as_rows(&self) -> Vec<Row> {
        let name = Row::new(["Name", self.name.as_str()]);
        let amount: Row = Row::new(["Amount".to_string(), self.amount.to_string()]);

        let rows = vec![name, amount];

        rows
    }
}
