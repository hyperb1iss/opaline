//! Deep ratatui integration — `OpalineColor` and `OpalineStyle` become
//! first-class ratatui citizens.
//!
//! All 9 ratatui modifiers are supported. `OpalineStyle` implements `Styled`,
//! granting the full `Stylize` fluent API (`.bold()`, `.fg()`, etc.).
//! `ThemeRatatuiExt` provides one-call `Span`, `Line`, and `Text` builders.

use std::borrow::Cow;

use ratatui_core::style::{Color, Modifier, Style, Styled};
#[cfg(feature = "gradients")]
use ratatui_core::text::Span;
use ratatui_core::text::{Line, Text};

use crate::color::OpalineColor;
#[cfg(feature = "gradients")]
use crate::gradient::Gradient;
use crate::style::OpalineStyle;
use crate::theme::Theme;

// ═══════════════════════════════════════════════════════════════════════════════
// Color conversion
// ═══════════════════════════════════════════════════════════════════════════════

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

/// An `OpalineColor` can be used directly as a foreground `Style`.
impl From<OpalineColor> for Style {
    fn from(c: OpalineColor) -> Self {
        Style::default().fg(Color::Rgb(c.r, c.g, c.b))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Style conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineStyle> for Style {
    fn from(s: OpalineStyle) -> Self {
        let mut style = Style::default();

        if let Some(fg) = s.fg {
            style = style.fg(Color::Rgb(fg.r, fg.g, fg.b));
        }
        if let Some(bg) = s.bg {
            style = style.bg(Color::Rgb(bg.r, bg.g, bg.b));
        }

        let modifiers = Modifier::from_bits_truncate(s.modifier_bits());
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

// ═══════════════════════════════════════════════════════════════════════════════
// Styled — unlocks the full Stylize fluent API on OpalineStyle
// ═══════════════════════════════════════════════════════════════════════════════

impl Styled for OpalineStyle {
    type Item = Style;

    fn style(&self) -> Style {
        Style::from(self)
    }

    fn set_style<S: Into<Style>>(self, style: S) -> Style {
        Style::from(self).patch(style)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ThemeRatatuiExt — zero-friction theme → ratatui bridge
// ═══════════════════════════════════════════════════════════════════════════════

/// Convenience methods on `Theme` for direct ratatui type access.
///
/// One-call methods for `Color`, `Style`, `Span`, `Line`, and `Text` —
/// no intermediate types, no manual wrapping.
pub trait ThemeRatatuiExt {
    /// Get a ratatui `Color` for a token name.
    fn ratatui_color(&self, token: &str) -> Color;

    /// Get a ratatui `Style` for a named style.
    fn ratatui_style(&self, name: &str) -> Style;

    /// Create a styled `Span` from a token name and content.
    fn ratatui_span<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> ratatui_core::text::Span<'a>;

    /// Create a styled `Line` from a token name and content.
    fn ratatui_line<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> Line<'a>;

    /// Create a styled `Text` from a token name and content.
    fn ratatui_text<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> Text<'a>;

    /// Sample a named gradient at position `t` and return a ratatui `Color`.
    #[cfg(feature = "gradients")]
    fn ratatui_gradient(&self, name: &str, t: f32) -> Color;

    /// Create a `Line` with per-character gradient coloring.
    #[cfg(feature = "gradients")]
    fn gradient_styled_line(&self, gradient_name: &str, content: &str) -> Line<'static>;
}

impl ThemeRatatuiExt for Theme {
    fn ratatui_color(&self, token: &str) -> Color {
        self.color(token).into()
    }

    fn ratatui_style(&self, name: &str) -> Style {
        self.style(name).into()
    }

    fn ratatui_span<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> ratatui_core::text::Span<'a> {
        ratatui_core::text::Span::styled(content, self.ratatui_style(style_name))
    }

    fn ratatui_line<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> Line<'a> {
        Line::styled(content, self.ratatui_style(style_name))
    }

    fn ratatui_text<'a>(&self, style_name: &str, content: impl Into<Cow<'a, str>>) -> Text<'a> {
        Text::styled(content, self.ratatui_style(style_name))
    }

    #[cfg(feature = "gradients")]
    fn ratatui_gradient(&self, name: &str, t: f32) -> Color {
        self.gradient(name, t).into()
    }

    #[cfg(feature = "gradients")]
    fn gradient_styled_line(&self, gradient_name: &str, content: &str) -> Line<'static> {
        if let Some(gradient) = self.get_gradient(gradient_name) {
            Line::from(gradient_spans(content, gradient))
        } else {
            Line::raw(content.to_string())
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradient helpers
// ═══════════════════════════════════════════════════════════════════════════════

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

/// Render text with per-character gradient coloring, returning a `Line`.
#[cfg(feature = "gradients")]
pub fn gradient_text_line(text: &str, gradient: &Gradient) -> Line<'static> {
    Line::from(gradient_spans(text, gradient))
}

/// Render a repeated character bar with gradient coloring, returning a `Line`.
#[cfg(feature = "gradients")]
pub fn gradient_bar(width: usize, ch: char, gradient: &Gradient) -> Line<'static> {
    Line::from(gradient_line(width, ch, gradient))
}
