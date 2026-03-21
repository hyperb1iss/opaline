//! Crossterm adapter — direct terminal styling without ratatui.
//!
//! Provides `From` conversions for `OpalineColor` → [`Color`] and
//! `OpalineStyle` → [`ContentStyle`], plus gradient helpers that produce
//! [`StyledContent`] sequences for per-character gradient rendering.
//!
//! ```rust,ignore
//! use crossterm::style::{ContentStyle, Stylize};
//! use opaline::{Theme, OpalineColor};
//!
//! let theme = Theme::default();
//! let style = ContentStyle::from(theme.style("keyword"));
//! print!("{}", style.apply("fn"));
//! ```

use std::fmt::Display;

use crossterm::style::{Attribute, Color, ContentStyle, StyledContent};

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
        Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

impl From<&OpalineColor> for Color {
    fn from(c: &OpalineColor) -> Self {
        Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Style conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineStyle> for ContentStyle {
    fn from(s: OpalineStyle) -> Self {
        let mut style = Self::new();

        style.foreground_color = s.fg.map(Into::into);
        style.background_color = s.bg.map(Into::into);

        if s.bold {
            style.attributes.set(Attribute::Bold);
        }
        if s.dim {
            style.attributes.set(Attribute::Dim);
        }
        if s.italic {
            style.attributes.set(Attribute::Italic);
        }
        if s.underline {
            style.attributes.set(Attribute::Underlined);
        }
        if s.slow_blink {
            style.attributes.set(Attribute::SlowBlink);
        }
        if s.rapid_blink {
            style.attributes.set(Attribute::RapidBlink);
        }
        if s.reversed {
            style.attributes.set(Attribute::Reverse);
        }
        if s.hidden {
            style.attributes.set(Attribute::Hidden);
        }
        if s.crossed_out {
            style.attributes.set(Attribute::CrossedOut);
        }

        style
    }
}

impl From<&OpalineStyle> for ContentStyle {
    fn from(s: &OpalineStyle) -> Self {
        s.clone().into()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme helpers
// ═══════════════════════════════════════════════════════════════════════════════

impl Theme {
    /// Apply a named style to content, producing a crossterm [`StyledContent`].
    pub fn crossterm_styled<D: Display>(
        &self,
        style_name: &str,
        content: D,
    ) -> StyledContent<D> {
        ContentStyle::from(self.style(style_name)).apply(content)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradient helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Render a string with per-character gradient coloring as crossterm
/// [`StyledContent`] values. Each element implements `Display`.
#[cfg(feature = "gradients")]
pub fn gradient_styled(text: &str, gradient: &Gradient) -> Vec<StyledContent<String>> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return vec![];
    }

    let colors = gradient.generate(chars.len());
    chars
        .into_iter()
        .zip(colors)
        .map(|(ch, color)| {
            let style = ContentStyle {
                foreground_color: Some(color.into()),
                ..ContentStyle::new()
            };
            style.apply(ch.to_string())
        })
        .collect()
}

/// Render a repeated character across `width` with gradient coloring.
#[cfg(feature = "gradients")]
pub fn gradient_bar(width: usize, ch: char, gradient: &Gradient) -> Vec<StyledContent<String>> {
    if width == 0 {
        return vec![];
    }

    let colors = gradient.generate(width);
    colors
        .into_iter()
        .map(|color| {
            let style = ContentStyle {
                foreground_color: Some(color.into()),
                ..ContentStyle::new()
            };
            style.apply(ch.to_string())
        })
        .collect()
}
