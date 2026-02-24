use ratatui_core::style::{Color, Modifier, Style};
#[cfg(feature = "gradients")]
use ratatui_core::text::Span;

use crate::color::OpalineColor;
#[cfg(feature = "gradients")]
use crate::gradient::Gradient;
use crate::style::OpalineStyle;
use crate::theme::Theme;

// ── Color conversion ─────────────────────────────────────────────────────

impl From<OpalineColor> for Color {
    fn from(c: OpalineColor) -> Self {
        Color::Rgb(c.r, c.g, c.b)
    }
}

impl From<&OpalineColor> for Color {
    fn from(c: &OpalineColor) -> Self {
        Color::Rgb(c.r, c.g, c.b)
    }
}

// ── Style conversion ─────────────────────────────────────────────────────

impl From<OpalineStyle> for Style {
    fn from(s: OpalineStyle) -> Self {
        let mut style = Style::default();

        if let Some(fg) = s.fg {
            style = style.fg(Color::Rgb(fg.r, fg.g, fg.b));
        }
        if let Some(bg) = s.bg {
            style = style.bg(Color::Rgb(bg.r, bg.g, bg.b));
        }

        let mut modifiers = Modifier::empty();
        if s.bold {
            modifiers |= Modifier::BOLD;
        }
        if s.italic {
            modifiers |= Modifier::ITALIC;
        }
        if s.underline {
            modifiers |= Modifier::UNDERLINED;
        }
        if s.dim {
            modifiers |= Modifier::DIM;
        }

        if !modifiers.is_empty() {
            style = style.add_modifier(modifiers);
        }

        style
    }
}

impl From<&OpalineStyle> for Style {
    fn from(s: &OpalineStyle) -> Self {
        s.clone().into()
    }
}

// ── Theme extension trait ────────────────────────────────────────────────

/// Convenience methods on `Theme` for direct `ratatui` type access.
pub trait ThemeRatatuiExt {
    /// Get a ratatui `Color` for a token name.
    fn ratatui_color(&self, token: &str) -> Color;

    /// Get a ratatui `Style` for a named style.
    fn ratatui_style(&self, name: &str) -> Style;

    /// Sample a named gradient at position `t` and return a ratatui `Color`.
    #[cfg(feature = "gradients")]
    fn ratatui_gradient(&self, name: &str, t: f32) -> Color;
}

impl ThemeRatatuiExt for Theme {
    fn ratatui_color(&self, token: &str) -> Color {
        self.color(token).into()
    }

    fn ratatui_style(&self, name: &str) -> Style {
        self.style(name).into()
    }

    #[cfg(feature = "gradients")]
    fn ratatui_gradient(&self, name: &str, t: f32) -> Color {
        self.gradient(name, t).into()
    }
}

// ── Gradient helpers ─────────────────────────────────────────────────────

/// Render a string with per-character gradient coloring, producing a `Vec<Span>`.
#[cfg(feature = "gradients")]
pub fn gradient_spans(text: &str, gradient: &Gradient) -> Vec<Span<'static>> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return vec![];
    }

    let colors = gradient.generate(chars.len());
    chars
        .into_iter()
        .zip(colors)
        .map(|(ch, color)| {
            Span::styled(
                ch.to_string(),
                Style::default().fg(Color::Rgb(color.r, color.g, color.b)),
            )
        })
        .collect()
}

/// Render a repeated character across `width` with gradient coloring.
#[cfg(feature = "gradients")]
pub fn gradient_line(width: usize, ch: char, gradient: &Gradient) -> Vec<Span<'static>> {
    if width == 0 {
        return vec![];
    }

    let colors = gradient.generate(width);
    colors
        .into_iter()
        .map(|color| {
            Span::styled(
                ch.to_string(),
                Style::default().fg(Color::Rgb(color.r, color.g, color.b)),
            )
        })
        .collect()
}
