use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, List, ListState},
    Frame,
};

use crate::dataframe::DataType;

#[derive(Default, Debug, Clone)]
pub struct DTypeSelect {
    pub state: ListState,
}

impl DTypeSelect {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        let block = Block::new()
            .borders(Borders::all())
            .title("change column datatype")
            .title_style(Style::default().dim());
        let items = vec![
            // DataType::Null,
            DataType::Bool,
            DataType::Int,
            DataType::Float,
            DataType::Date,
            DataType::String,
        ];
        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().reversed());
        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn to_dtype(&self) -> DataType {
        if let Some(idx) = self.state.selected() {
            match idx {
                // 0 => DataType::Null,
                0 => DataType::Bool,
                1 => DataType::Int,
                2 => DataType::Float,
                3 => DataType::Date,
                _ => DataType::String,
            }
        } else {
            DataType::String
        }
    }
}
