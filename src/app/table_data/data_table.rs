use std::path::PathBuf;

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Text, ToLine},
    widgets::{self, Block, Borders, Paragraph, Table, TableState},
    Frame,
};

#[allow(unused)]
use tracing::info;

use crate::event::{
    csv::{CsvData, CsvDescription},
    io_task::IoTask,
};

use super::{popup::Popup, RowsExt};

#[derive(Default, Debug, Clone)]
pub struct DataTable {
    headers: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
    pub table_state: TableState,
    pub buffer: String,
    // Cell indicies (column , row)
    pub editing: Option<(usize, usize)>,
    pub path: Option<PathBuf>,
    pub delim: char,
    pub is_dirty: bool,
    pub parse_errors: Vec<String>,
}

impl DataTable {
    pub fn set_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }
    pub fn set_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }
    pub fn set_parse_errors(mut self, parse_errors: Vec<String>) -> Self {
        self.parse_errors = parse_errors;
        self
    }
    pub fn set_path(mut self, path: Option<PathBuf>) -> Self {
        self.path = path;
        self
    }
    pub fn set_delim(mut self, delim: char) -> Self {
        self.delim = delim;
        self
    }
}

impl DataTable {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [top, bottom] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Max(self.parse_errors.len() as u16),
            ],
        )
        .areas(area);

        let table = self.rat_table();
        frame.render_stateful_widget(table, top, &mut self.table_state);

        if let Some((popup, popup_area)) = self.popup(top) {
            frame.render_widget(popup, popup_area);
        }

        if !self.parse_errors.is_empty() {
            let lines = self.parse_errors.iter().map(|e| e.to_line()).collect_vec();
            let par = Paragraph::new(lines).red();
            frame.render_widget(par, bottom);
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
        let bottom_title = match !self.is_dirty && !self.parse_errors.is_empty() {
            true => String::from("Parsed with errors"),
            false => String::new(),
        };

        let path = match self.is_dirty {
            false => format!("{:?}", self.path),
            true => {
                format!("*{:?}", self.path)
            }
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title_bottom(bottom_title)
            .title(format!(
                "{path:} - {:?} - {:?}",
                self.editing,
                self.table_state.selected_cell()
            ))
            .title_style(Style::default().light_green());

        let header_row = self.rat_row_header();
        let data_rows = self.rat_rows();
        let _widths = self.equal_column_widths();
        let widths = self.min_column_widths();
        let table = Table::new(data_rows, widths)
            .header(header_row)
            // .row_highlight_style(Style::new())
            // .column_highlight_style(Style::new())
            .cell_highlight_style(Style::new().bold().reversed());
        table.block(block)
    }
    fn min_column_widths(&self) -> Vec<Constraint> {
        let widths = self.rows.column_widths_min(self.header_widths());
        widths.into_iter().map(Constraint::Length).collect_vec()
    }
    fn equal_column_widths(&self) -> Vec<Constraint> {
        let cols = self.width();
        let equal: u16 = (100 / cols) as u16;
        let mut width_constraints = vec![];
        for _ in 0..cols {
            width_constraints.push(Constraint::Percentage(equal));
        }
        width_constraints
    }

    fn popup(&self, area: Rect) -> Option<(Popup<'static>, Rect)> {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        if let Some((row, col)) = self.editing {
            let popup = Popup::default()
                .content(self.buffer.clone())
                .style(Style::new().yellow())
                .title(self.cell_get_header(col))
                .title_bottom(format!("row = {}, column = {}", row, col,))
                .title_style(Style::new().white().bold())
                .border_style(Style::new().red());
            Some((popup, popup_area))
        } else {
            None
        }
    }
    fn set_dirty(&mut self) {
        self.is_dirty = true;
        self.parse_errors = vec![];
    }
    fn cell_set_row_col(&mut self, row: usize, col: usize, content: &str) {
        if self.rows.is_valid_coords(row, col) {
            self.rows.set_content(row, col, content);
        }
        self.set_dirty();
    }
    fn cell_get_row_col(&self, row: usize, col: usize) -> String {
        if self.rows.is_valid_coords(row, col) {
            self.rows.get_owned(row, col).unwrap_or_default()
        } else {
            // should never happen
            String::new()
        }
    }
    fn cell_get_header(&self, col: usize) -> String {
        if col <= self.width() {
            self.headers.get(col).unwrap().to_owned()
        } else {
            String::new()
        }
    }
    pub fn append_row(&mut self) {
        self.rows.push(vec![String::new(); self.width()]);
        // info!("{:#?}", self);
    }
}

impl DataTable {
    pub fn toggle_edit(&mut self) {
        // IF EDITING
        if let Some((row, col)) = self.editing {
            let buf = self.buffer.clone();
            // Set selected cell value to buffer & clear buffer
            self.cell_set_row_col(row, col, &buf);
            self.editing = None;
            self.buffer = String::new();
        } else
        // IF NOT EDITING
        {
            // Set editing to selected cell
            self.editing = self.table_state.selected_cell();
            if let Some((row, col)) = self.editing {
                self.buffer = self.cell_get_row_col(row, col)
            }
        }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
    pub fn width(&self) -> usize {
        self.headers.len()
    }
    pub fn has_data(&self) -> bool {
        self.height() > 0 && self.width() > 0
    }
    // fn dimensions(&self) -> (usize, usize) {
    //     (self.width(), self.height())
    // }
    fn header_widths(&self) -> Vec<u16> {
        self.headers
            .clone()
            .into_iter()
            .map(|h| h.len() as u16)
            .collect_vec()
    }
    // fn cell_rect(&self) -> Rect {
    //     Rect::new(0, 0, self.width() as u16, self.height() as u16)
    // }
    pub fn action_save(&self) -> IoTask {
        let data = CsvData {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        };
        IoTask::SaveCsv(CsvDescription {
            data,
            delim: self.delim,
            errors: vec![],
            path: self.path.clone(),
        })
    }
}
