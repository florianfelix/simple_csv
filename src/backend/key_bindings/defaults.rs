use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;

use crate::app::evt_handlers::Action;

pub fn default_keymap_normal() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [
        (
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()),
            Action::Quit,
        ),
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            Action::ToggleEdit,
        ),
        (
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
            Action::Save,
        ),
        (
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::ALT),
            Action::SaveKeyBindings,
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
        (
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::empty()),
            Action::AppendRow,
        ),
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::empty()),
            Action::AppendColumn,
        ),
    ];
    IndexMap::from_iter(map)
}
pub fn default_keymap_edit() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            Action::ToggleEdit,
        ),
        (
            KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
            Action::ConfirmSelectRight,
        ),
        (
            KeyEvent::new(KeyCode::Left, KeyModifiers::empty()),
            Action::ConfirmSelectLeft,
        ),
        (
            KeyEvent::new(KeyCode::Up, KeyModifiers::empty()),
            Action::ConfirmSelectRight,
        ),
        (
            KeyEvent::new(KeyCode::Down, KeyModifiers::empty()),
            Action::ConfirmSelectDown,
        ),
    ];
    IndexMap::from_iter(map)
}
