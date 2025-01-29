use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn triple_pane(a: u16, b: u16, c: u16, area: Rect) -> [Rect; 3] {
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
