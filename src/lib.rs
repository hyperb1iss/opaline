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
//! | `builtin-themes` | yes | Embed builtin TOML themes via `include_str!` |
//! | `gradients` | yes | Multi-stop gradient support |
//! | `ratatui` | yes | `From` impls for `ratatui::style::{Color, Style}` |
//! | `global-state` | no | Process-wide `current()`/`set_theme()` singleton |
//! | `discovery` | no | Load user themes from `~/.config/<app>/themes/` |
//! | `named-colors` | no | CSS named color parsing |

pub mod color;
pub mod error;
pub mod style;

#[cfg(feature = "gradients")]
pub mod gradient;

pub mod schema;
pub mod resolver;
pub mod loader;
pub mod theme;

pub mod adapters;

#[cfg(feature = "builtin-themes")]
pub mod builtins;

#[cfg(feature = "discovery")]
pub mod discovery;

// ── Re-exports ───────────────────────────────────────────────────────────

pub use color::{ColorParseError, OpalineColor};
pub use error::OpalineError;
pub use style::OpalineStyle;
pub use theme::Theme;

#[cfg(feature = "gradients")]
pub use gradient::Gradient;

#[cfg(feature = "ratatui")]
pub use adapters::ratatui::ThemeRatatuiExt;

#[cfg(feature = "global-state")]
pub use theme::{current, load_theme, set_theme};

#[cfg(all(feature = "global-state", feature = "builtin-themes"))]
pub use theme::load_theme_by_name;
