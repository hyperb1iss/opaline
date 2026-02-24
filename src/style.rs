use crate::color::OpalineColor;

/// A composed style with optional foreground/background colors and text modifiers.
///
/// Styles are the output of the resolution pipeline — they reference resolved
/// colors (not token names) and can be converted to `ratatui::style::Style`
/// via the adapter.
///
/// All 9 ratatui modifiers are supported: `bold`, `dim`, `italic`, `underline`,
/// `slow_blink`, `rapid_blink`, `reversed`, `hidden`, and `crossed_out`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OpalineStyle {
    pub fg: Option<OpalineColor>,
    pub bg: Option<OpalineColor>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub dim: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub slow_blink: bool,
    #[serde(default)]
    pub rapid_blink: bool,
    #[serde(default)]
    pub reversed: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub crossed_out: bool,
}

impl OpalineStyle {
    /// Create an empty style with no colors or modifiers.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            slow_blink: false,
            rapid_blink: false,
            reversed: false,
            hidden: false,
            crossed_out: false,
        }
    }

    /// Create a style with only a foreground color.
    pub const fn fg(color: OpalineColor) -> Self {
        Self {
            fg: Some(color),
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            slow_blink: false,
            rapid_blink: false,
            reversed: false,
            hidden: false,
            crossed_out: false,
        }
    }

    /// Create a style with only a background color.
    pub const fn bg(color: OpalineColor) -> Self {
        Self {
            fg: None,
            bg: Some(color),
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            slow_blink: false,
            rapid_blink: false,
            reversed: false,
            hidden: false,
            crossed_out: false,
        }
    }

    /// Set the foreground color.
    #[must_use]
    pub const fn with_fg(mut self, color: OpalineColor) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub const fn with_bg(mut self, color: OpalineColor) -> Self {
        self.bg = Some(color);
        self
    }

    /// Enable bold.
    #[must_use]
    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enable dim.
    #[must_use]
    pub const fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Enable italic.
    #[must_use]
    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enable underline.
    #[must_use]
    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Enable slow blink.
    #[must_use]
    pub const fn slow_blink(mut self) -> Self {
        self.slow_blink = true;
        self
    }

    /// Enable rapid blink.
    #[must_use]
    pub const fn rapid_blink(mut self) -> Self {
        self.rapid_blink = true;
        self
    }

    /// Enable reversed (inverted fg/bg).
    #[must_use]
    pub const fn reversed(mut self) -> Self {
        self.reversed = true;
        self
    }

    /// Enable hidden.
    #[must_use]
    pub const fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Enable crossed out (strikethrough).
    #[must_use]
    pub const fn crossed_out(mut self) -> Self {
        self.crossed_out = true;
        self
    }

    /// Merge another style on top of this one.
    ///
    /// The `other` style takes precedence for colors where set.
    /// Boolean modifiers are OR'd together.
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            fg: other.fg.or(self.fg),
            bg: other.bg.or(self.bg),
            bold: self.bold || other.bold,
            dim: self.dim || other.dim,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            slow_blink: self.slow_blink || other.slow_blink,
            rapid_blink: self.rapid_blink || other.rapid_blink,
            reversed: self.reversed || other.reversed,
            hidden: self.hidden || other.hidden,
            crossed_out: self.crossed_out || other.crossed_out,
        }
    }

    /// Returns a `Modifier` bitfield of all active modifiers.
    ///
    /// This is a convenience for building adapter conversions.
    pub(crate) fn modifier_bits(&self) -> u16 {
        let mut bits: u16 = 0;
        if self.bold {
            bits |= 0b0000_0000_0001;
        }
        if self.dim {
            bits |= 0b0000_0000_0010;
        }
        if self.italic {
            bits |= 0b0000_0000_0100;
        }
        if self.underline {
            bits |= 0b0000_0000_1000;
        }
        if self.slow_blink {
            bits |= 0b0000_0001_0000;
        }
        if self.rapid_blink {
            bits |= 0b0000_0010_0000;
        }
        if self.reversed {
            bits |= 0b0000_0100_0000;
        }
        if self.hidden {
            bits |= 0b0000_1000_0000;
        }
        if self.crossed_out {
            bits |= 0b0001_0000_0000;
        }
        bits
    }
}
