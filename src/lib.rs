//! # Opaline
//!
//! A token-based theme engine for [Ratatui](https://ratatui.rs) TUI applications.
//!
//! Opaline gives any Ratatui app TOML-driven themes with a three-layer resolution
//! pipeline: **palette** (raw hex colors) → **tokens** (semantic names) → **styles**
//! (composed fg/bg + modifiers). Themes can also define multi-stop **gradients**.
//!
//! ## Quick start
//!
//! ```rust
//! use opaline::{Theme, OpalineColor};
//!
//! // Load the default builtin theme
//! let theme = Theme::default();
//!
//! // Access semantic tokens
//! let accent = theme.color("accent.primary");
//! let style = theme.style("keyword");
//! ```
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `builtin-themes` | yes | 13 embedded TOML themes via `include_str!` |
//! | `gradients` | yes | Multi-stop gradient support |
//! | `ratatui` | yes | `From` impls for `ratatui::style::{Color, Style}` |
//! | `cli` | no | `colored` crate adapter for ANSI terminal output |
//! | `global-state` | no | Process-wide `current()`/`set_theme()` singleton |
//! | `discovery` | no | Load user themes from `~/.config/<app>/themes/` |

pub mod color;
pub mod error;
pub mod style;

#[cfg(feature = "gradients")]
pub mod gradient;

pub mod loader;
pub mod resolver;
pub mod schema;
pub mod theme;

pub mod adapters;
pub mod names;

#[cfg(feature = "builtin-themes")]
pub mod builtins;

#[cfg(feature = "discovery")]
pub mod discovery;

// ── Re-exports ───────────────────────────────────────────────────────────

// Core types — always available
pub use color::{ColorParseError, OpalineColor};
pub use error::OpalineError;
pub use loader::{load_from_file, load_from_str};
pub use schema::{StyleDef, ThemeFile, ThemeMeta, ThemeVariant};
pub use style::OpalineStyle;
pub use theme::{Theme, ThemeBuilder};

// Gradients
#[cfg(feature = "gradients")]
pub use gradient::Gradient;

// Ratatui adapter
#[cfg(all(feature = "ratatui", feature = "gradients"))]
pub use adapters::ratatui::{gradient_bar, gradient_line, gradient_spans, gradient_text_line};

// CLI adapter
#[cfg(all(feature = "cli", feature = "gradients"))]
pub use adapters::cli::gradient_string;
#[cfg(feature = "cli")]
pub use adapters::cli::{ColoredExt, ThemeCliExt};

// Global state
#[cfg(feature = "global-state")]
pub use theme::{current, load_theme, set_theme};
#[cfg(all(feature = "global-state", feature = "builtin-themes"))]
pub use theme::{load_theme_by_name, load_theme_by_name_with};
#[cfg(all(
    feature = "global-state",
    feature = "builtin-themes",
    feature = "discovery"
))]
pub use theme::{load_theme_by_name_for_app, load_theme_by_name_for_app_with};

// Builtins
#[cfg(feature = "builtin-themes")]
pub use builtins::{ThemeInfo, list_available_themes, load_by_name};

// Discovery
#[cfg(feature = "discovery")]
pub use discovery::{app_theme_dirs, theme_dirs};
