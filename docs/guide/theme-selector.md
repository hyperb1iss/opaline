# ThemeSelector Widget

Opaline ships a ready-to-use theme picker widget for Ratatui apps. It provides a searchable, scrollable theme list with a live color preview pane — users see exactly how each theme looks before committing.

## Feature Flag

The widget requires the `widgets` feature:

```toml
[dependencies]
opaline = { version = "0.1", features = ["widgets"] }
```

This enables `global-state` and `builtin-themes` automatically, and pulls in full `ratatui` (with crossterm) rather than just `ratatui-core`.

## Quick Start

```rust
use opaline::{ThemeSelector, ThemeSelectorState, ThemeSelectorAction};

// 1. Create state when opening the picker
let mut state = ThemeSelectorState::with_current_selected();

// 2. In your key handler
match state.handle_key(key_event) {
    ThemeSelectorAction::Select(id) => {
        // Theme already applied — just save the preference
        save_user_preference(&id);
        close_picker();
    }
    ThemeSelectorAction::Cancel => {
        // Original theme automatically restored
        close_picker();
    }
    _ => {} // Navigate, FilterChanged, Noop
}

// 3. In your render function
frame.render_stateful_widget(ThemeSelector::new(), area, &mut state);
```

## Types

### `ThemeSelectorAction`

Returned by `handle_key()`:

| Variant | Meaning |
|---------|---------|
| `Navigate` | Cursor moved, live preview applied |
| `Select(String)` | Enter pressed — contains the theme's kebab-case ID |
| `Cancel` | Esc pressed — original theme restored |
| `FilterChanged` | Filter text changed, list recomputed |
| `Noop` | Key not handled by the selector |

### `ThemeSelectorState`

Owns all widget state. Create on open, drop on close.

```rust
// All themes, no pre-selection
let state = ThemeSelectorState::new();

// Pre-selects the currently active global theme
let state = ThemeSelectorState::with_current_selected();

// With app-level token derivation for live preview
let state = ThemeSelectorState::with_current_selected()
    .with_derive(my_app::derive_tokens);
```

**Methods:**
- `handle_key(KeyEvent) -> ThemeSelectorAction` — process keyboard input
- `selected_theme() -> Option<&ThemeInfo>` — currently highlighted theme's metadata
- `filter() -> &str` — current filter text

### `ThemeSelector`

The stateful widget. Implements `StatefulWidget`.

```rust
let widget = ThemeSelector::new();
let widget = ThemeSelector::new().title("Pick a Color Scheme");
```

## Layout

The widget renders a two-pane layout:

```
┌─ Select Theme ──────────────────────────────────────────────┐
│ Filter: cat                                                 │
│─────────────────────────────┬───────────────────────────────│
│ ▌ Dark Themes ▐             │ Catppuccin Mocha              │
│   Catppuccin Mocha          │ by Catppuccin                 │
│ > Dracula                   │                               │
│   Gruvbox Dark              │ Soothing pastel theme for the │
│   Kanagawa Wave             │ high-spirited!                │
│ ▌ Light Themes ▐            │                               │
│   Catppuccin Latte          │ ████████ ████████ ████████    │
│   Everforest Light          │ primary  secondary tertiary   │
│                             │ ████████ ████████ ████████    │
│                             │ success  warning  error       │
│                             │                               │
│                             │ ▓▓▒▒░░██▓▓▒▒░░██▓▓▒▒░░██    │
│ ↑↓/jk Navigate  Enter OK   │ primary gradient              │
│ Esc Cancel                  │                               │
└─────────────────────────────┴───────────────────────────────┘
```

- **Left pane (55%)** — filter input + scrollable theme list with dark/light section headers
- **Right pane (45%)** — theme name, author, description, 6 color swatches, gradient bar

## Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `k` | Move cursor up |
| `↓` / `j` | Move cursor down |
| `Enter` | Confirm selection |
| `Esc` | Cancel (restore original theme) |
| Any character | Append to filter |
| `Backspace` | Delete last filter character |

## Live Preview

The widget applies each theme to the global state as you navigate — your entire app re-renders with the previewed theme in real-time. On cancel (`Esc`), the original theme is restored from a snapshot taken at construction time.

## With App Derivation

If your app uses [derived tokens](./derivation.md), pass your derivation function so previews include your app-specific tokens:

```rust
fn derive_tokens(theme: &mut opaline::Theme) {
    let primary = theme.color("accent.primary");
    theme.register_default_token("sidebar.bg", primary.darken(0.85));
}

let state = ThemeSelectorState::with_current_selected()
    .with_derive(derive_tokens);
```

Without this, live preview would show the raw theme without your computed tokens — potentially missing colors or incorrect styling.

## Integration Example

A minimal integration into a Ratatui app with a modal theme picker:

```rust
use crossterm::event::{self, KeyCode};
use opaline::{ThemeSelector, ThemeSelectorAction, ThemeSelectorState};

struct App {
    theme_picker: Option<ThemeSelectorState>,
}

impl App {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        if let Some(picker) = &mut self.theme_picker {
            match picker.handle_key(key) {
                ThemeSelectorAction::Select(id) => {
                    // Theme is already applied — persist the choice
                    self.save_theme_preference(&id);
                    self.theme_picker = None;
                }
                ThemeSelectorAction::Cancel => {
                    // Original theme was restored automatically
                    self.theme_picker = None;
                }
                _ => {}
            }
        } else if key.code == KeyCode::Char('t') {
            // Open theme picker
            self.theme_picker = Some(
                ThemeSelectorState::with_current_selected()
            );
        }
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        if let Some(state) = &mut self.theme_picker {
            frame.render_stateful_widget(
                ThemeSelector::new(),
                frame.area(),
                state,
            );
        }
    }
}
```
