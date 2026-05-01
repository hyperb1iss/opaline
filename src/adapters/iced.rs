//! iced adapter — apply Opaline themes to iced GUIs.
//!
//! Provides `From` conversions for [`Color`](iced_core::Color) and helpers
//! that map theme tokens onto iced's [`Palette`](iced_core::theme::Palette)
//! and [`Custom`](iced_core::theme::Custom) theme.
//!
//! ```rust,ignore
//! use iced::Theme;
//! use std::sync::Arc;
//!
//! let theme = opaline::Theme::default();
//! let custom = opaline::adapters::iced::to_iced_custom(&theme);
//! let iced_theme = Theme::Custom(Arc::new(custom));
//! ```

use iced_core::Color;
use iced_core::theme::palette::Extended;
use iced_core::theme::{Custom, Palette};

use crate::color::OpalineColor;
use crate::names::tokens;
use crate::theme::Theme;

// ═══════════════════════════════════════════════════════════════════════════════
// Color conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineColor> for Color {
    fn from(c: OpalineColor) -> Self {
        Color::from_rgb8(c.r, c.g, c.b)
    }
}

impl From<&OpalineColor> for Color {
    fn from(c: &OpalineColor) -> Self {
        Color::from_rgb8(c.r, c.g, c.b)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme → Palette / Custom
// ═══════════════════════════════════════════════════════════════════════════════

/// Convert an Opaline [`Theme`] to an iced [`Palette`].
///
/// Maps the six required iced palette slots from the standard Opaline token
/// contract:
///
/// | iced field   | Opaline token     |
/// |--------------|-------------------|
/// | `background` | `bg.base`         |
/// | `text`       | `text.primary`    |
/// | `primary`    | `accent.primary`  |
/// | `success`    | `success`         |
/// | `warning`    | `warning`         |
/// | `danger`     | `error`           |
pub fn to_iced_palette(theme: &Theme) -> Palette {
    Palette {
        background: theme.color(tokens::BG_BASE).into(),
        text: theme.color(tokens::TEXT_PRIMARY).into(),
        primary: theme.color(tokens::ACCENT_PRIMARY).into(),
        success: theme.color(tokens::SUCCESS).into(),
        warning: theme.color(tokens::WARNING).into(),
        danger: theme.color(tokens::ERROR).into(),
    }
}

/// Convert an Opaline [`Theme`] to an iced [`Extended`] palette.
///
/// Builds a [`Palette`] via [`to_iced_palette`] and lets iced derive the
/// extended tints via [`Extended::generate`].
pub fn to_iced_extended(theme: &Theme) -> Extended {
    Extended::generate(to_iced_palette(theme))
}

/// Convert an Opaline [`Theme`] to an iced [`Custom`] theme.
///
/// Uses [`Theme::meta.name`](crate::schema::ThemeMeta) as the iced theme
/// name and derives the extended palette via iced's default generator.
/// Drop the result into [`iced::Theme::Custom`](https://docs.rs/iced/latest/iced/enum.Theme.html#variant.Custom)
/// (wrapped in `Arc`) to use it.
pub fn to_iced_custom(theme: &Theme) -> Custom {
    Custom::new(theme.meta.name.clone(), to_iced_palette(theme))
}
