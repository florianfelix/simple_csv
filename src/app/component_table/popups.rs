use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use super::{extensions::BufferExt, DataTable};

impl DataTable {
    pub fn render_popup_edit_cell(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        frame.render_widget(Clear, popup_area);
        let [left, right] = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(70), Constraint::Fill(1)],
        )
        .areas(popup_area);
        let title_bottom = match self.active_header {
            Some(ref header) => header.to_string(),
            None => "No Header Selected".to_string(),
        };
        let block = Block::new()
            .borders(Borders::all())
            .title_bottom(title_bottom)
            .title_style(Style::default().dim());
        let txt = Paragraph::new(self.textbuffer.to_cursor_string())
            .wrap(Wrap { trim: true })
            .block(block);

        frame.render_widget(txt, left);
        if let Some(sk) = &mut self.skim {
            sk.render(frame, right);
        }
    }

    pub fn render_popup_edit(&mut self, frame: &mut Frame, area: Rect) {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        frame.render_widget(Clear, popup_area);
        let block = Block::new().borders(Borders::all());
        let txt = Paragraph::new(self.textbuffer.to_cursor_string())
            .wrap(Wrap { trim: true })
            .block(block);

        frame.render_widget(txt, popup_area);
    }
}
