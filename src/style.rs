use crate::color::OpalineColor;

/// A composed style with optional foreground/background colors and text modifiers.
///
/// Styles are the output of the resolution pipeline — they reference resolved
/// colors (not token names) and can be converted to `ratatui::style::Style`
/// via the adapter.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OpalineStyle {
    pub fg: Option<OpalineColor>,
    pub bg: Option<OpalineColor>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
}

impl OpalineStyle {
    /// Create an empty style with no colors or modifiers.
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
        }
    }

    /// Create a style with only a foreground color.
    pub const fn fg(color: OpalineColor) -> Self {
        Self {
            fg: Some(color),
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            dim: false,
        }
    }

    /// Create a style with only a background color.
    pub const fn bg(color: OpalineColor) -> Self {
        Self {
            fg: None,
            bg: Some(color),
            bold: false,
            italic: false,
            underline: false,
            dim: false,
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

    /// Enable dim.
    #[must_use]
    pub const fn dim(mut self) -> Self {
        self.dim = true;
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
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            dim: self.dim || other.dim,
        }
    }
}
