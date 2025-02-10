use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Row, Table, TableState},
    Frame,
};

use crate::backend::key_bindings::KeyBindings;

#[derive(Default, Debug)]
pub struct KeyBindingsDisplay {
    state_left: TableState,
    state_right: TableState,
}

impl KeyBindingsDisplay {
    pub fn render(&mut self, frame: &mut Frame, area: Rect, keys: &KeyBindings) {
        let [left, right] = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .areas(area);
        let data = keys.display();
        let rows = data.0.into_iter().map(Row::new).collect_vec();
        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths).header(Row::new(["ACTION", "KEY"]));

        let [top, bottom] = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Fill(1)],
        )
        .areas(left);
        let title = Line::from("Normal Map").centered().bold();
        frame.render_widget(title, top);
        frame.render_stateful_widget(table, bottom, &mut self.state_left);

        let rows = data.1.into_iter().map(Row::new).collect_vec();
        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths).header(Row::new(["ACTION", "KEY"]));

        let [top, bottom] = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Fill(1)],
        )
        .areas(right);
        let title = Line::from("Edit Map").centered().bold();
        frame.render_widget(title, top);
        frame.render_stateful_widget(table, bottom, &mut self.state_right);
    }
}
