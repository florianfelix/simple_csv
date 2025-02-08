use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;

use crate::app::evt_handlers::Action;

pub fn default_keymap_normal() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            Action::ToggleEdit,
        ),
        (
            KeyEvent::new(
                KeyCode::Char('s'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
            Action::Save,
        ),
        (
            KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
            Action::SelectRight,
        ),
        (
            KeyEvent::new(KeyCode::Left, KeyModifiers::empty()),
            Action::SelectLeft,
        ),
        (
            KeyEvent::new(KeyCode::Up, KeyModifiers::empty()),
            Action::SelectUp,
        ),
        (
            KeyEvent::new(KeyCode::Down, KeyModifiers::empty()),
            Action::SelectDown,
        ),
    ];
    IndexMap::from_iter(map)
}
pub fn default_keymap_edit() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
        Action::ToggleEdit,
    )];
    IndexMap::from_iter(map)
}
