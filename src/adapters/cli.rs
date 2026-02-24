//! CLI adapter for terminal output via the [`colored`](https://docs.rs/colored) crate.
//!
//! Provides extension traits to apply theme colors and styles to strings as
//! ANSI true-color escape sequences, plus gradient rendering for CLI output.

use colored::{ColoredString, Colorize};

use crate::color::OpalineColor;
#[cfg(feature = "gradients")]
use crate::gradient::Gradient;
use crate::style::OpalineStyle;
use crate::theme::Theme;

// ── String coloring ─────────────────────────────────────────────────────

/// Extension trait for applying theme colors to any string-like type.
pub trait ColoredExt {
    /// Apply a theme color as foreground.
    fn theme_fg(self, color: OpalineColor) -> ColoredString;

    /// Apply a theme color as background.
    fn theme_bg(self, color: OpalineColor) -> ColoredString;

    /// Apply a full theme style (fg, bg, modifiers).
    fn theme_style(self, style: &OpalineStyle) -> ColoredString;
}

impl<S: AsRef<str>> ColoredExt for S {
    fn theme_fg(self, color: OpalineColor) -> ColoredString {
        self.as_ref().truecolor(color.r, color.g, color.b)
    }

    fn theme_bg(self, color: OpalineColor) -> ColoredString {
        self.as_ref().on_truecolor(color.r, color.g, color.b)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn theme_style(self, style: &OpalineStyle) -> ColoredString {
        let mut result: ColoredString = self.as_ref().into();

        if let Some(fg) = style.fg {
            result = result.truecolor(fg.r, fg.g, fg.b);
        }
        if let Some(bg) = style.bg {
            result = result.on_truecolor(bg.r, bg.g, bg.b);
        }
        if style.bold {
            result = result.bold();
        }
        if style.italic {
            result = result.italic();
        }
        if style.underline {
            result = result.underline();
        }
        if style.dim {
            result = result.dimmed();
        }

        result
    }
}

// ── Theme extension trait ───────────────────────────────────────────────

/// Convenience methods on `Theme` for direct CLI colored output.
pub trait ThemeCliExt {
    /// Get a token color as an RGB tuple for use with `.truecolor()`.
    fn cli_rgb(&self, token: &str) -> (u8, u8, u8);

    /// Apply a token color as foreground on text.
    fn cli_colored(&self, text: &str, token: &str) -> ColoredString;

    /// Apply a named gradient across a string, returning ANSI-escaped output.
    #[cfg(feature = "gradients")]
    fn cli_gradient(&self, text: &str, gradient_name: &str) -> String;
}

impl ThemeCliExt for Theme {
    fn cli_rgb(&self, token: &str) -> (u8, u8, u8) {
        self.color(token).into()
    }

    fn cli_colored(&self, text: &str, token: &str) -> ColoredString {
        let color = self.color(token);
        text.truecolor(color.r, color.g, color.b)
    }

    #[cfg(feature = "gradients")]
    fn cli_gradient(&self, text: &str, gradient_name: &str) -> String {
        if let Some(gradient) = self.get_gradient(gradient_name) {
            gradient_string(text, gradient)
        } else {
            text.to_string()
        }
    }
}

// ── Gradient rendering ──────────────────────────────────────────────────

/// Render a string with per-character gradient coloring as ANSI escape codes.
#[cfg(feature = "gradients")]
#[allow(clippy::cast_precision_loss, clippy::as_conversions)]
pub fn gradient_string(text: &str, gradient: &Gradient) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let colors = gradient.generate(chars.len());
    let mut result = String::new();

    for (ch, color) in chars.into_iter().zip(colors) {
        let colored = ch.to_string().truecolor(color.r, color.g, color.b);
        result.push_str(&colored.to_string());
    }

    result
}
