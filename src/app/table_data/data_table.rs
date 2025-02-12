use std::path::PathBuf;

use indexmap::IndexMap;
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
use crate::backend::{
    file_formats::{
        file_csv::{CsvData, CsvDescription},
        file_toml::TomlDescription,
    },
    tasks::events::IoCommand,
};

#[derive(Default, Debug, Clone)]
pub enum EditTarget {
    #[default]
    None,
    Cell((usize, usize)),
    Header(usize),
    FileName,
}

#[derive(Debug)]
pub struct DataTable {
    pub(crate) headers: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
    pub table_state: TableState,
    pub textbuffer: text_buffer::Buffer,
    pub edit_target: EditTarget,
    pub path: Option<PathBuf>,
    pub delim: char,
    pub is_dirty: bool,
    pub parse_errors: Vec<String>,
}

impl Default for DataTable {
    fn default() -> Self {
        Self {
            headers: vec![String::from("key"), String::from("value")],
            rows: vec![vec![String::from(""), String::from("")]],
            table_state: TableState::default(),
            textbuffer: Buffer::new(),
            edit_target: EditTarget::None,
            path: None,
            delim: ';',
            is_dirty: true,
            parse_errors: vec![],
        }
    }
}

impl DataTable {
    pub fn from_csv_description(&mut self, csv_description: CsvDescription) {
        self.headers = csv_description.data.headers;
        self.rows = csv_description.data.rows;
        self.table_state = TableState::default();
        self.textbuffer = Buffer::new();
        self.edit_target = EditTarget::None;
        self.path = csv_description.path;
        self.delim = csv_description.delim;
        self.is_dirty = false;
        self.parse_errors = csv_description.errors;
    }
    // pub fn new_simple() -> Self {
    //     let mut new = Self::default();
    //     new.append_column_named("key");
    //     new.append_column_named("value");
    //     new.append_row();
    //     new.select_cell_right();
    //     new.path = Some(PathBuf::from("file.csv"));
    //     new
    // }
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
        let path = match self.is_dirty {
            false => self.path.to_cursor_string().to_string(),
            true => {
                format!("*{:}", self.path.to_cursor_string())
            }
        };

        let buf = self.textbuffer.to_string();

        let _debug_title = format!(
            "{path:} - {:?} - {:?} -Buf: {} -Cursor {}",
            self.edit_target,
            self.table_state.selected_cell(),
            buf,
            self.textbuffer.cursor().chars()
        );

        let title = format!("{path:}  -  help(<?>)");
        let bottom_title = match self.edit_target {
            EditTarget::None => String::from(
                "help: ?, new column: c, rename column: h, new row: r, rename file: f, save: ctrl-s, quit: q or ctrl-c",
            ),
            _ => String::from("accept: enter"),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title_bottom(bottom_title)
            .title(title)
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
            EditTarget::Cell((_row, col)) => {
                let popup = Popup::default()
                    .content(self.textbuffer.to_cursor_string())
                    .style(Style::new().yellow())
                    .title(self.cell_get_header(col))
                    .title_bottom("Edit cell".to_string())
                    .title_style(Style::new().white().bold())
                    .border_style(Style::new().red());
                Some((popup, popup_area))
            }
            EditTarget::Header(col) => {
                let popup = Popup::default()
                    .content(self.textbuffer.to_cursor_string())
                    .style(Style::new().yellow())
                    .title(self.cell_get_header(col))
                    .title_bottom("Edit column name".to_string())
                    .title_style(Style::new().white().bold())
                    .border_style(Style::new().red());
                Some((popup, popup_area))
            }
            EditTarget::FileName => {
                let popup = Popup::default()
                    .content(self.textbuffer.to_cursor_string())
                    .style(Style::new().yellow())
                    // .title(self.cell_get_header(col))
                    .title_bottom("Edit filename".to_string())
                    .title_style(Style::new().white().bold())
                    .border_style(Style::new().red());
                Some((popup, popup_area))
            }
            EditTarget::None => None,
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
    pub fn append_column_named(&mut self, name: &str) {
        self.headers.push(String::from(name));
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
    pub fn edit_file_name(&mut self) {
        if let Some(path) = self.path.clone() {
            self.edit_target = EditTarget::FileName;
            self.textbuffer = Buffer::from(path.to_string_lossy().into_owned());
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
        } else {
            self.edit_target = EditTarget::FileName;
            self.textbuffer = Buffer::new();
        }
    }
    pub fn apply_edit(&mut self) {
        match self.edit_target {
            EditTarget::Header(col) => self.set_column_name(col, self.textbuffer.to_string()),
            EditTarget::Cell((row, col)) => {
                self.cell_set_row_col(row, col, self.textbuffer.to_string())
            }
            EditTarget::FileName => {
                if self.textbuffer.is_empty() {
                    self.path = None;
                } else {
                    self.path = Some(self.textbuffer.to_string().into());
                };
            }
            EditTarget::None => {}
        }
        self.edit_target = EditTarget::None;
        self.textbuffer = Buffer::new();
    }
    pub fn edit_cancel(&mut self) {
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
    pub fn save_csv_command(&self) -> IoCommand {
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

    pub fn save_toml_command(&self) -> IoCommand {
        let data = self
            .rows
            .iter()
            .map(|row| {
                let mut map = IndexMap::new();
                row.iter().zip(self.headers.clone()).for_each(|(v, k)| {
                    map.insert(k, v.to_owned());
                });
                map
            })
            .collect_vec();
        IoCommand::SaveToml(TomlDescription { rows: data })
    }
}
