# Simple Csv Terminal Application
tui app to quickly create/edit small csv tables

## Usage
`scsv <filename> <delimiter>`

if <delimiter> is ommitted the default delimiter is <;> for now

### export
save: ctrl-s

export as toml: ctrl-t


## keys normal mode
| KEY | ACTION|
| - | - |
| q | Quit |
| ctrl-c | Quit |
| enter | EditCell |
| r | AppendRow |
| c | AppendColumn |
| h | EditHeader |
| f | EditFileName |
| ctrl-s | Save |
| ctrl-t | SaveToml |
| ctrl-k | SaveKeyBindings |
| right | SelectCellRight |
| left | SelectCellLeft |
| up | SelectCellUp |
| down | SelectCellDown |
| alt-right | MoveColumnRight |
| alt-left | MoveColumnLeft |
| alt-up | MoveRowUp |
| alt-down | MoveRowDown |
| s | SortByColumn |
| alt-s | SortByColumnReversed |
| "?" | ToggleKeyBindingsDisplay |

## keys edit mode
| KEY | ACTION|
| - | - |
| enter | ApplyEdit |
  | ctrl-enter | ConfirmSelectCellRight |
  | alt-enter | ConfirmSelectCellDown |
  | esc | CancelEdit |
  | tab | NextSuggestion |
  | down | NextSuggestion |
  | up | PreviousSuggestion |
  | shift-right | ConfirmSelectCellRight |
  | shift-left | ConfirmSelectCellLeft |
  | shift-up | ConfirmSelectCellRight |
  | shift-down | ConfirmSelectCellDown |
  | right | CursorRight |
  | left | CursorLeft |

  ### keybindings file
  Save the default keybindings with ctrl-k into `$HOME/.config/simple_csv/keymap.toml`.
  The config file is hot reloaded.
