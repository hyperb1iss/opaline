//! owo-colors adapter — zero-allocation terminal coloring.
//!
//! Provides conversion from [`OpalineStyle`] to [`owo_colors::Style`] and
//! an extension trait for applying theme colors to any displayable type.
//!
//! ```rust,ignore
//! use opaline::adapters::owo_colors::OwoThemeExt;
//!
//! let theme = Theme::default();
//! println!("{}", "hello".style(theme.owo_style("keyword")));
//! ```

use owo_colors::Style;

use crate::style::OpalineStyle;
use crate::theme::Theme;
#[cfg(feature = "gradients")]
use unicode_segmentation::UnicodeSegmentation;

// ═══════════════════════════════════════════════════════════════════════════════
// Style conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineStyle> for Style {
    fn from(s: OpalineStyle) -> Self {
        build_owo_style(&s)
    }
}

impl From<&OpalineStyle> for Style {
    fn from(s: &OpalineStyle) -> Self {
        build_owo_style(s)
    }
}

fn build_owo_style(s: &OpalineStyle) -> Style {
    let mut style = Style::new();

    if let Some(fg) = s.fg {
        style = style.truecolor(fg.r, fg.g, fg.b);
    }
    if let Some(bg) = s.bg {
        style = style.on_truecolor(bg.r, bg.g, bg.b);
    }
    if s.bold {
        style = style.bold();
    }
    if s.dim {
        style = style.dimmed();
    }
    if s.italic {
        style = style.italic();
    }
    if s.underline {
        style = style.underline();
    }
    if s.slow_blink || s.rapid_blink {
        style = style.blink();
    }
    if s.reversed {
        style = style.reversed();
    }
    if s.hidden {
        style = style.hidden();
    }
    if s.crossed_out {
        style = style.strikethrough();
    }

    style
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme extension
// ═══════════════════════════════════════════════════════════════════════════════

/// Extension trait for themed owo-colors output on [`Theme`].
pub trait OwoThemeExt {
    /// Build an [`owo_colors::Style`] from a named theme style.
    fn owo_style(&self, style_name: &str) -> Style;

    /// Build an [`owo_colors::Style`] with a token color as foreground.
    fn owo_fg(&self, token: &str) -> Style;

    /// Build an [`owo_colors::Style`] with a token color as background.
    fn owo_bg(&self, token: &str) -> Style;
}

impl OwoThemeExt for Theme {
    fn owo_style(&self, style_name: &str) -> Style {
        Style::from(self.style(style_name))
    }

    fn owo_fg(&self, token: &str) -> Style {
        let c = self.color(token);
        Style::new().truecolor(c.r, c.g, c.b)
    }

    fn owo_bg(&self, token: &str) -> Style {
        let c = self.color(token);
        Style::new().on_truecolor(c.r, c.g, c.b)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradient helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Render a string with per-character gradient coloring as an ANSI string.
///
/// Uses owo-colors for zero-allocation formatting of each character.
#[cfg(feature = "gradients")]
pub fn gradient_string(text: &str, gradient: &crate::gradient::Gradient) -> String {
    use owo_colors::OwoColorize as _;
    use std::fmt::Write as _;

    let graphemes: Vec<&str> = text.graphemes(true).collect();
    if graphemes.is_empty() {
        return String::new();
    }

    let colors = gradient.generate(graphemes.len());
    let mut result = String::with_capacity(text.len() * 20);

    for (grapheme, color) in graphemes.into_iter().zip(colors) {
        let _ = write!(result, "{}", grapheme.truecolor(color.r, color.g, color.b));
    }

    result
}
