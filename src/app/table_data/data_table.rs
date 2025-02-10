use std::path::PathBuf;

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Text, ToLine},
    widgets::{self, Block, Borders, Paragraph, Table, TableState},
    Frame,
};

use text_buffer::Buffer;
#[allow(unused)]
use tracing::info;

use super::{extensions::BufferExt, extensions::RowsExt, popup::Popup};
use crate::backend::{tasks::events::IoCommand, CsvData, CsvDescription};

#[derive(Default, Debug, Clone)]
pub enum EditTarget {
    #[default]
    None,
    Cell((usize, usize)),
    Header(usize),
}

#[derive(Default, Debug)]
pub struct DataTable {
    headers: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
    pub table_state: TableState,
    pub textbuffer: text_buffer::Buffer,
    pub edit_target: EditTarget,
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

        if let Some((edit_popup, popup_area)) = self.edit_popup(top) {
            frame.render_widget(edit_popup, popup_area);
            // edit_popup.render(popup_area, frame.buffer_mut());
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

        let buf = self.textbuffer.to_string();

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title_bottom(bottom_title)
            .title(format!(
                "{path:} - {:?} - {:?} -Buf: {} -Cursor {}",
                self.edit_target,
                self.table_state.selected_cell(),
                buf,
                self.textbuffer.cursor().chars()
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

    fn edit_popup(&self, area: Rect) -> Option<(Popup<'static>, Rect)> {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };

        match self.edit_target {
            EditTarget::Cell((row, col)) => {
                let popup = Popup::default()
                    .content(self.textbuffer.to_cursor_string())
                    .style(Style::new().yellow())
                    .title(self.cell_get_header(col))
                    .title_bottom(format!("row = {}, column = {}", row, col,))
                    .title_style(Style::new().white().bold())
                    .border_style(Style::new().red());
                Some((popup, popup_area))
            }
            EditTarget::Header(col) => {
                let popup = Popup::default()
                    .content(self.textbuffer.to_cursor_string())
                    .style(Style::new().yellow())
                    .title(self.cell_get_header(col))
                    .title_bottom(format!("column = {}", col))
                    .title_style(Style::new().white().bold())
                    .border_style(Style::new().red());
                Some((popup, popup_area))
            }
            _ => None,
        }
    }
    fn set_dirty(&mut self) {
        self.is_dirty = true;
        self.parse_errors = vec![];
    }
    fn cell_set_row_col(&mut self, row: usize, col: usize, content: String) {
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
    pub fn append_column(&mut self) {
        self.headers.push(String::from("NewColumn"));
        self.rows.append_column();
    }
    fn set_column_name(&mut self, col: usize, content: String) {
        let value = self.headers.get_mut(col).unwrap();
        *value = content;
    }
    pub fn edit_column_name(&mut self) {
        if let Some(col) = self.table_state.selected_column() {
            self.edit_target = EditTarget::Header(col);
            self.textbuffer = Buffer::from(self.headers.get(col).unwrap().clone());
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
        }
    }
    pub fn edit_cell(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            self.edit_target = EditTarget::Cell((row, col));
            self.textbuffer = Buffer::from(self.cell_get_row_col(row, col));
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
        }
    }
    pub fn apply_edit(&mut self) {
        use EditTarget::*;
        match self.edit_target {
            Header(col) => self.set_column_name(col, self.textbuffer.to_string()),
            Cell((row, col)) => self.cell_set_row_col(row, col, self.textbuffer.to_string()),
            None => (),
        }
        self.edit_target = EditTarget::None;
        self.textbuffer = Buffer::new();
    }
    pub fn move_cursor_right(&mut self) {
        let current = self.textbuffer.cursor().chars();
        self.textbuffer.set_cursor(current + 1);
    }
    pub fn move_cursor_left(&mut self) {
        let current = self.textbuffer.cursor().chars();
        if let Some(new) = current.checked_sub(1) {
            self.textbuffer.set_cursor(new);
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.textbuffer.insert_char(c);
    }
    pub fn delete_backwards(&mut self) {
        self.textbuffer.delete_backwards(1);
    }
    pub fn delete_forwards(&mut self) {
        self.textbuffer.delete_forwards(1);
    }
}

impl DataTable {
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
    pub fn save_command(&self) -> IoCommand {
        let data = CsvData {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        };
        IoCommand::SaveCsv(CsvDescription {
            data,
            delim: self.delim,
            errors: vec![],
            path: self.path.clone(),
        })
    }
}
