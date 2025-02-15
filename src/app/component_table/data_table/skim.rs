use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

#[derive(Default, Debug, Clone)]
struct Choice {
    score: i64,
    text: String,
}

impl From<String> for Choice {
    fn from(value: String) -> Self {
        Self {
            score: 0,
            text: value,
        }
    }
}
impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        Self {
            score: 0,
            text: value.to_owned(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Skim {
    choices: Vec<Choice>,
    pattern: String,
    matches: Vec<String>,
    state: ListState,
}
impl Skim {
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::new().borders(Borders::all());
        let list = List::new(self.matches.clone())
            .block(block)
            .highlight_style(Style::default().reversed());
        frame.render_stateful_widget(list, area, &mut self.state);
    }
    pub fn select_next(&mut self) {
        if self.state.selected() == Some(self.matches.len() - 1) {
            self.state.select(None);
        } else {
            self.state.select_next();
        }
    }
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }
    pub fn selected(&self) -> Option<String> {
        if let Some(sel) = self.state.selected() {
            if let Some(s) = self.matches.get(sel) {
                return Some(s.to_owned());
            }
        }
        None
    }
    pub fn new(pattern: impl Into<String>, choices: Vec<String>) -> Self {
        Self {
            pattern: pattern.into(),
            choices: choices
                .iter()
                .map(|v| Choice::from(v.as_str()))
                .collect_vec(),
            matches: vec![],
            state: ListState::default(),
        }
    }
    pub fn update(&mut self, pattern: &str) {
        self.pattern = pattern.to_owned();
        self.fmatch();
        // if !self.matches.is_empty() && self.state.selected().is_none() {
        //     self.state.select(Some(0));
        // } else {
        //     self.state.select(None);
        // }
    }
    fn fmatch(&mut self) {
        let matcher = SkimMatcherV2::default();
        #[allow(clippy::manual_inspect)]
        let mut res = self
            .choices
            .iter_mut()
            .map(|c| {
                c.score = matcher
                    .fuzzy_match(&c.text, &self.pattern)
                    .unwrap_or_default();
                c
            })
            .filter(|c| c.score > 0)
            .sorted_unstable_by(|a, b| b.score.cmp(&a.score))
            .take(3)
            .map(|c| c.text.to_owned())
            .collect_vec();
        res.dedup();
        self.matches = res;
    }
}
