use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;

use super::action::Action;

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
