# Feature Flags

Opaline uses Cargo feature flags to keep the dependency tree lean. Enable only what you need.

## Default Features

These are enabled by default with `opaline = "0.2"`:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `builtin-themes` | 39 embedded TOML themes via `include_str!` | None |
| `gradients` | Multi-stop gradient support (`Gradient` type) | `unicode-segmentation 1.12` |
| `ratatui` | `From` impls, inherent `span()`/`line()`/`text()`/`gradient_text()` on `Theme` | `ratatui-core 0.1` |

## Optional Features

These must be explicitly enabled:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `cli` | `colored` crate adapter — `ThemeCliExt`, `ColoredExt`, `gradient_string` | `colored 3` |
| `crossterm` | Direct crossterm adapter — `Color`, `ContentStyle`, gradient helpers | `crossterm 0.29` |
| `owo-colors` | Zero-allocation terminal coloring — `Style` conversion, `OwoThemeExt` | `owo-colors 4` |
| `css` | CSS custom properties + classes generation from tokens/styles/gradients | None |
| `syntect` | Syntax highlighting theme generation — `Color`, `StyleModifier`, `Theme` | `syntect 5` |
| `egui` | Immediate-mode GUI adapter — `Color32`, `Visuals` from theme tokens | `egui 0.33` |
| `global-state` | Process-wide theme singleton — `current()`, `set_theme()` | `parking_lot 0.12` |
| `discovery` | User theme directory scanning — `app_theme_dirs()`, `theme_dirs()` | `dirs 6` |
| `widgets` | Theme selector widget with live preview | `ratatui 0.30`, `crossterm 0.29`, `unicode-width 0.2` (enables `global-state` + `builtin-themes`) |

## Configuration Examples

```toml
# Default — builtins + gradients + ratatui
[dependencies]
opaline = "0.2"

# Minimal — just the core engine, no builtins or adapters
[dependencies]
opaline = { version = "0.2", default-features = false }

# Core + gradients only (for non-Ratatui use)
[dependencies]
opaline = { version = "0.2", default-features = false, features = ["gradients"] }

# CLI tool (colored output, no TUI)
[dependencies]
opaline = { version = "0.2", features = ["cli"] }

# Direct crossterm styling
[dependencies]
opaline = { version = "0.2", features = ["crossterm"] }

# Zero-allocation terminal coloring
[dependencies]
opaline = { version = "0.2", features = ["owo-colors"] }

# CSS generation for web frameworks (Leptos, Yew, Dioxus, Tauri)
[dependencies]
opaline = { version = "0.2", features = ["css"] }

# Syntax highlighting theme generation
[dependencies]
opaline = { version = "0.2", features = ["syntect"] }

# egui GUI theming
[dependencies]
opaline = { version = "0.2", features = ["egui"] }

# Full TUI app with global state
[dependencies]
opaline = { version = "0.2", features = ["global-state"] }

# User-configurable themes
[dependencies]
opaline = { version = "0.2", features = ["discovery"] }

# TUI app with theme picker widget
[dependencies]
opaline = { version = "0.2", features = ["widgets"] }

# Everything
[dependencies]
opaline = { version = "0.2", features = [
    "builtin-themes", "gradients", "ratatui",
    "cli", "crossterm", "owo-colors", "css",
    "syntect", "egui",
    "global-state", "discovery", "widgets"
] }
```

## Feature Interactions

Some features gate additional API when combined:

| Combination | Unlocks |
|-------------|---------|
| `ratatui` + `gradients` | `theme.gradient_text()`, `gradient_spans()`, `gradient_line()`, `gradient_bar()`, `gradient_text_line()` |
| `cli` + `gradients` | `gradient_string()` (colored crate) |
| `crossterm` + `gradients` | `gradient_styled()`, `gradient_bar()` |
| `owo-colors` + `gradients` | `gradient_string()` (owo-colors) |
| `css` + `gradients` | Gradient CSS custom properties as `linear-gradient()` |
| `global-state` + `builtin-themes` | `load_theme_by_name()`, `load_theme_by_name_with()` |
| `builtin-themes` + `discovery` | `list_available_themes_for_app()`, `list_available_themes_in_dirs()` |
| `global-state` + `builtin-themes` + `discovery` | `load_theme_by_name_for_app()`, `load_theme_by_name_for_app_with()`, `load_theme_by_name_in_dirs()` |

## Without Default Features

With `default-features = false`, you get:

- `OpalineColor`, `OpalineStyle`, `OpalineError`
- `Theme`, `ThemeBuilder`
- `load_from_str(toml, None)`, `load_from_file(path)`
- Schema types (`ThemeFile`, `ThemeMeta`, `StyleDef`)
- Resolver pipeline

You lose:

- No builtin themes (must provide your own TOML)
- No gradient support
- No Ratatui type conversions
