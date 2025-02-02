use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Table},
    Frame,
};

use super::data_row::DataRow;

#[derive(Default, Debug, Clone)]
pub struct DataTable {
    data_rows: Vec<DataRow>,
}

impl DataTable {
    pub fn example() -> Self {
        Self {
            data_rows: DataRow::examples(),
        }
    }
}

impl DataTable {
    pub fn render_table(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Table");

        let widths = self.equal_row_widths();
        let rows = self.data_rows.iter().map(|r| r.rat_row()).collect_vec();
        let table = Table::new(rows, widths).block(block);

        frame.render_widget(table, area);
    }
    fn equal_row_widths(&self) -> Vec<Constraint> {
        if !self.data_rows.is_empty() {
            let n = self.data_rows.first().unwrap().headers().len();
            let equal: u16 = (100 / n) as u16;
            let mut v = vec![];
            for _ in 0..n {
                v.push(Constraint::Percentage(equal));
            }
            v
        } else {
            vec![]
        }
    }
}
