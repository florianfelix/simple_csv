use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Table, TableState},
    Frame,
};

use super::data_row::DataRow;

#[derive(Default, Debug, Clone)]
pub struct DataTable {
    data_rows: Vec<DataRow>,
    pub table_state: TableState,
    pub buffer: String,
    pub editing: Option<(usize, usize)>,
}

impl DataTable {
    pub fn example() -> Self {
        Self {
            data_rows: DataRow::examples(),
            table_state: TableState::default(),
            buffer: String::new(),
            editing: None,
        }
    }
}

impl DataTable {
    pub fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!("Table - {}", self.buffer));

        let widths = self.equal_row_widths();
        let rows = self.data_rows.iter().map(|r| r.rat_row()).collect_vec();
        let table = Table::new(rows, widths)
            .block(block)
            .row_highlight_style(Style::new().reversed())
            .cell_highlight_style(Style::new().bold().red());

        frame.render_stateful_widget(table, area, &mut self.table_state);
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
    pub fn toggle_edit(&mut self) {
        if let Some((x, y)) = self.editing {
            // apply buffer
            self.editing = None
        } else {
            self.editing = self.table_state.selected_cell();
            if let Some((x, y)) = self.editing {
                // fill buffer
            }
        }
    }
    pub fn select_cell_next(&mut self) {
        if let Some((x, y)) = self.table_state.selected_cell() {
            let y: usize = {
                let new = y + 1;
                if new >= self.width() {
                    0
                } else {
                    new
                }
            };
            self.table_state.select_cell(Some((x, y)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    pub fn select_cell_previous(&mut self) {
        if let Some((x, y)) = self.table_state.selected_cell() {
            let y: usize = {
                if y == 0 {
                    self.width()
                } else {
                    y - 1
                }
            };
            self.table_state.select_cell(Some((x, y)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    fn active(&self) {
        if let Some(idx) = self.table_state.selected() {
            // self.table_state.cell
        }
    }
    fn height(&self) -> usize {
        self.data_rows.len()
    }
    fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }
        self.data_rows.first().unwrap().len()
    }
}
