# Feature Flags

Opaline uses Cargo feature flags to keep the dependency tree lean. Enable only what you need.

## Default Features

These are enabled by default with `opaline = "0.1"`:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `builtin-themes` | 20 embedded TOML themes via `include_str!` | None |
| `gradients` | Multi-stop gradient support (`Gradient` type) | None |
| `ratatui` | `From` impls for `ratatui::style::{Color, Style}`, `ThemeRatatuiExt` | `ratatui-core 0.1` |

## Optional Features

These must be explicitly enabled:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `cli` | `colored` crate adapter — `ThemeCliExt`, `ColoredExt`, `gradient_string` | `colored 3` |
| `global-state` | Process-wide theme singleton — `current()`, `set_theme()` | `parking_lot 0.12` |
| `discovery` | User theme directory scanning — `app_theme_dirs()`, `theme_dirs()` | `dirs 6` |

## Configuration Examples

```toml
# Default — builtins + gradients + ratatui
[dependencies]
opaline = "0.1"

# Minimal — just the core engine, no builtins or adapters
[dependencies]
opaline = { version = "0.1", default-features = false }

# Core + gradients only (for non-Ratatui use)
[dependencies]
opaline = { version = "0.1", default-features = false, features = ["gradients"] }

# CLI tool (colored output, no TUI)
[dependencies]
opaline = { version = "0.1", features = ["cli"] }

# Full TUI app with global state
[dependencies]
opaline = { version = "0.1", features = ["global-state"] }

# User-configurable themes
[dependencies]
opaline = { version = "0.1", features = ["discovery"] }

# Everything
[dependencies]
opaline = { version = "0.1", features = [
    "builtin-themes", "gradients", "ratatui",
    "cli", "global-state", "discovery"
] }
```

## Feature Interactions

Some features gate additional API when combined:

| Combination | Unlocks |
|-------------|---------|
| `ratatui` + `gradients` | `gradient_spans()`, `gradient_line()`, `gradient_bar()`, `gradient_text_line()` |
| `cli` + `gradients` | `gradient_string()` |
| `global-state` + `builtin-themes` | `load_theme_by_name()` |

## Without Default Features

With `default-features = false`, you get:

- `OpalineColor`, `OpalineStyle`, `OpalineError`
- `Theme`, `ThemeBuilder`
- `load_from_str()`, `load_from_file()`
- Schema types (`ThemeFile`, `ThemeMeta`, `StyleDef`)
- Resolver pipeline

You lose:

- No builtin themes (must provide your own TOML)
- No gradient support
- No Ratatui type conversions
