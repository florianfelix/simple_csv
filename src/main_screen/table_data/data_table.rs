use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Position, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{self, Block, Borders, Table, TableState},
    Frame,
};

use super::io::headers_rows_from_csv_string;

#[derive(Default, Debug, Clone)]
pub struct DataTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    pub table_state: TableState,
    pub buffer: String,
    // Cell indicies (column , row)
    pub editing: Option<(usize, usize)>,
}

impl DataTable {
    pub fn example() -> Self {
        let input = include_str!("sample.csv");
        let (headers, rows) = headers_rows_from_csv_string(input, ';');
        Self {
            headers,
            rows,
            table_state: TableState::default(),
            buffer: String::new(),
            editing: None,
        }
    }
}

impl DataTable {
    pub fn rat_row_header(&self) -> widgets::Row<'static> {
        let cells = self
            .headers
            .iter()
            .map(|s| widgets::Cell::new(Text::raw(s.to_owned())))
            .collect_vec();
        widgets::Row::new(cells)
    }
    pub fn rat_rows(&self) -> Vec<widgets::Row<'static>> {
        let mut rows = vec![];
        for r in self.rows.iter() {
            let cells = r
                .iter()
                .map(|s| widgets::Cell::new(s.to_owned()))
                .collect_vec();
            let row = widgets::Row::new(cells);
            rows.push(row);
        }
        rows
    }
    pub fn rat_table(&self) -> widgets::Table<'static> {
        let header_row = self.rat_row_header();
        let data_rows = self.rat_rows();
        let widths = self.equal_percentages();
        let table = Table::new(data_rows, widths)
            .header(header_row)
            .row_highlight_style(Style::new().reversed())
            .cell_highlight_style(Style::new().bold().fg(Color::DarkGray).bg(Color::LightCyan));
        table
    }
    fn equal_percentages(&self) -> Vec<Constraint> {
        let cols = self.rows.first().unwrap().len();
        let equal: u16 = (100 / cols) as u16;
        let mut width_constraints = vec![];
        for _ in 0..cols {
            width_constraints.push(Constraint::Percentage(equal));
        }
        width_constraints
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!("Table - {} - {:?}", self.buffer, self.editing));
        let table = self.rat_table().block(block);
        frame.render_stateful_widget(table, area, &mut self.table_state);
    }
    fn cell_set_col_row(&mut self, cell_col_row: (usize, usize), content: &str) {
        let (y, x) = cell_col_row;
        if x <= self.width() && y <= self.height() {
            let row = self.rows.get_mut(y).expect("row index out of bounds");
            let value = row.get_mut(x).expect("out of bounds");
            *value = String::from(content);
        }
    }
    fn cell_get_col_row(&self, cell_col_row: (usize, usize)) -> String {
        let (y, x) = cell_col_row;
        let area = self.rect();
        let inside = area.contains(Position::new(x as u16, y as u16));
        if inside {
            let row = self.rows.get(y).expect("row index out of bounds");
            row.get(x).unwrap().to_owned()
        } else {
            String::new()
        }
    }
}

impl DataTable {
    pub fn toggle_edit(&mut self) {
        // IF EDITING
        if let Some(col_row) = self.editing {
            let buf = self.buffer.clone();
            // Set selected cell value to buffer & clear buffer
            self.cell_set_col_row(col_row, &buf);
            self.editing = None;
            self.buffer = String::new();
        } else
        // IF NOT EDITING
        {
            // Set editing to selected cell
            self.editing = self.table_state.selected_cell();
            if let Some(row_col) = self.editing {
                self.buffer = self.cell_get_col_row(row_col)
            }
        }
    }
    pub fn select_cell_next(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let row: usize = {
                let new = row + 1;
                if new >= self.width() {
                    0
                } else {
                    new
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    pub fn select_cell_previous(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let row: usize = {
                if row == 0 {
                    self.width()
                } else {
                    row - 1
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn width(&self) -> usize {
        self.headers.len()
    }
    // fn dimensions(&self) -> (usize, usize) {
    //     (self.width(), self.height())
    // }
    fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width() as u16, self.height() as u16)
    }
}
