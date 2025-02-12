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
            Action::EditCell,
        ),
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT),
            Action::EditHeader,
        ),
        (
            KeyEvent::new(KeyCode::Char('f'), KeyModifiers::empty()),
            Action::EditFileName,
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
            Action::SelectCellDown,
        ),
        (
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::empty()),
            Action::AppendRow,
        ),
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::empty()),
            Action::AppendColumn,
        ),
        (
            KeyEvent::new(KeyCode::Char('?'), KeyModifiers::empty()),
            Action::ToggleKeyBindingsDisplay,
        ),
    ];
    IndexMap::from_iter(map)
}
pub fn default_keymap_edit() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            Action::ApplyEdit,
        ),
        (
            KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),
            Action::CancelEdit,
        ),
        (
            KeyEvent::new(KeyCode::Right, KeyModifiers::SHIFT),
            Action::ConfirmSelectRight,
        ),
        (
            KeyEvent::new(KeyCode::Left, KeyModifiers::SHIFT),
            Action::ConfirmSelectLeft,
        ),
        (
            KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT),
            Action::ConfirmSelectRight,
        ),
        (
            KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT),
            Action::ConfirmSelectCellDown,
        ),
        (
            KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
            Action::CursorRight,
        ),
        (
            KeyEvent::new(KeyCode::Left, KeyModifiers::empty()),
            Action::CursorLeft,
        ),
    ];
    IndexMap::from_iter(map)
}
