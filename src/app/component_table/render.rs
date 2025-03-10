use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Text, ToLine},
    widgets::{self, Block, Borders, Paragraph, Table},
    Frame,
};

use super::{extensions::BufferExt, DataTable, EditTarget};

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

        match self.edit_target {
            EditTarget::Cell((_, _)) => self.render_popup_edit_cell(frame, area),
            EditTarget::FileName => self.render_popup_edit(frame, area),
            EditTarget::Header(_) => self.render_popup_edit(frame, area),
            EditTarget::ColumnType(_) => self.render_popup_dtype_select(frame, area),
            EditTarget::None => {}
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
            .df
            .headers()
            .iter()
            .map(|s| widgets::Cell::new(Text::raw(s.name().to_owned())))
            .collect_vec();
        widgets::Row::new(cells).bold()
    }
    pub fn rat_row_footer(&self) -> widgets::Row<'static> {
        let cells = self
            .df
            .headers()
            .iter()
            .map(|s| widgets::Cell::new(Text::raw(s.dtype().to_string())))
            .collect_vec();
        widgets::Row::new(cells).bold()
    }
    pub fn rat_rows(&self) -> Vec<widgets::Row<'static>> {
        let mut rows = vec![];
        for (i, r) in self.df.rows().iter().enumerate() {
            let cells = r
                .iter()
                .map(|s| widgets::Cell::new(s.print()))
                .collect_vec();
            // let row = widgets::Row::new(cells);
            let row = if i % 2 == 1 {
                widgets::Row::new(cells).style(Style::default().dim())
            } else {
                widgets::Row::new(cells)
            };
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

        let dtypecol = match self.active_header() {
            Some(h) => format!("{:?}", h.dtype()),
            None => String::new(),
        };
        let pos = match self.table_state.selected_cell() {
            Some((row, col)) => format!("Row: {}, Col: {}", row, col),
            None => String::new(),
        };
        let title = format!("{path:} - Cell <{}> - Column type <{}>", pos, dtypecol);
        let bottom_title = match self.edit_target {
            EditTarget::None => String::from(
                "help: ?, new column: c, rename column: v, new row: r, rename file: f, save: ctrl-s, quit: q or ctrl-c",
            ),
            _ => String::from("accept: enter"),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title_bottom(bottom_title)
            .title(title)
            .title_style(Style::default().light_green());

        let table = Table::new(self.rat_rows(), self.min_column_widths())
            .header(self.rat_row_header())
            .footer(self.rat_row_footer())
            .row_highlight_style(Style::default().cyan().bold())
            // .column_highlight_style(Style::new())
            .cell_highlight_style(Style::new().bold().reversed());
        table.block(block)
    }
    fn min_column_widths(&self) -> Vec<Constraint> {
        let widths = self.df.min_column_widths();
        widths.into_iter().map(Constraint::Length).collect_vec()
    }
    #[allow(unused)]
    fn equal_column_widths(&self) -> Vec<Constraint> {
        let cols = self.df.width();
        let equal: u16 = (100 / cols) as u16;
        let mut width_constraints = vec![];
        for _ in 0..cols {
            width_constraints.push(Constraint::Percentage(equal));
        }
        width_constraints
    }
}
