//! Reusable theme widgets for Ratatui applications.
//!
//! Requires the `widgets` feature flag.

mod theme_selector;

pub use theme_selector::{ThemeSelector, ThemeSelectorAction, ThemeSelectorState};

#[doc(hidden)]
pub use theme_selector::wrap_text;
