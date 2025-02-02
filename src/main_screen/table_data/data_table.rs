use std::usize;

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Position, Rect},
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
            .title(format!("Table - {} - {:?}", self.buffer, self.editing));

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
    fn set(&mut self, position: (usize, usize), content: &str) {
        let (y, x) = position;
        if x <= self.width() && y <= self.height() {
            let row = self.data_rows.get_mut(y).unwrap();
            row.set_idx(x, content);
        }
    }
    fn get(&self, position: (usize, usize)) -> String {
        let (y, x) = position;
        let r = Rect::new(0, 0, self.width() as u16, self.height() as u16);
        let inside = r.contains(Position::new(x as u16, y as u16));
        if inside {
            let row = self.data_rows.get(y).unwrap();
            row.get_idx(x)
        } else {
            String::new()
        }
    }
    pub fn toggle_edit(&mut self) {
        // IF EDITING
        if let Some(cell_position) = self.editing {
            let buf = self.buffer.clone();
            // Set selected cell value to buffer & clear buffer
            self.set(cell_position, &buf);
            self.editing = None;
            self.buffer = String::new();
        } else
        // IF NOT EDITING
        {
            // Set editing to selected cell
            self.editing = self.table_state.selected_cell();
            if let Some(cell_position) = self.editing {
                self.buffer = self.get(cell_position)
            }
        }
    }
    pub fn select_cell_next(&mut self) {
        if let Some((y, x)) = self.table_state.selected_cell() {
            let x: usize = {
                let new = x + 1;
                if new >= self.width() {
                    0
                } else {
                    new
                }
            };
            self.table_state.select_cell(Some((y, x)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    pub fn select_cell_previous(&mut self) {
        if let Some((y, x)) = self.table_state.selected_cell() {
            let x: usize = {
                if x == 0 {
                    self.width()
                } else {
                    x - 1
                }
            };
            self.table_state.select_cell(Some((y, x)));
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
    fn diemansions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}
