use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn header_body_footer_areas(header_height: u16, footer_height: u16, area: Rect) -> [Rect; 3] {
    let [header, body, footer] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(header_height),
            Constraint::Fill(1),
            Constraint::Length(footer_height),
        ])
        .areas(area);
    [header, body, footer]
}

pub fn triple_pane_percantages(a: u16, b: u16, c: u16, area: Rect) -> [Rect; 3] {
    let [left, center, right] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(a),
            Constraint::Percentage(b),
            Constraint::Percentage(c),
        ])
        .areas(area);
    [left, center, right]
}
