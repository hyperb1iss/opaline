//! Syntect adapter — generate syntax highlighting themes from Opaline.
//!
//! Maps Opaline themes to [`syntect::highlighting::Theme`], bridging the
//! token-based theme engine into the syntax highlighting ecosystem used by
//! bat, delta, and many other tools.
//!
//! ```rust,ignore
//! use opaline::adapters::syntect::to_syntect_theme;
//!
//! let theme = opaline::Theme::default();
//! let syntect_theme = to_syntect_theme(&theme);
//! // Use with syntect's HighlightLines, html module, etc.
//! ```

use syntect::highlighting::{
    Color, FontStyle, ScopeSelectors, StyleModifier, Theme as SyntectTheme,
    ThemeItem, ThemeSettings,
};

use crate::color::OpalineColor;
use crate::style::OpalineStyle;
use crate::theme::Theme;

// ═══════════════════════════════════════════════════════════════════════════════
// Color conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineColor> for Color {
    fn from(c: OpalineColor) -> Self {
        Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 255,
        }
    }
}

impl From<&OpalineColor> for Color {
    fn from(c: &OpalineColor) -> Self {
        Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 255,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Style conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineStyle> for StyleModifier {
    fn from(s: OpalineStyle) -> Self {
        style_to_modifier(&s)
    }
}

impl From<&OpalineStyle> for StyleModifier {
    fn from(s: &OpalineStyle) -> Self {
        style_to_modifier(s)
    }
}

fn style_to_modifier(s: &OpalineStyle) -> StyleModifier {
    let mut font_style = FontStyle::empty();
    if s.bold {
        font_style |= FontStyle::BOLD;
    }
    if s.italic {
        font_style |= FontStyle::ITALIC;
    }
    if s.underline {
        font_style |= FontStyle::UNDERLINE;
    }

    StyleModifier {
        foreground: s.fg.map(Into::into),
        background: s.bg.map(Into::into),
        font_style: if font_style.is_empty() {
            None
        } else {
            Some(font_style)
        },
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme generation
// ═══════════════════════════════════════════════════════════════════════════════

/// Mapping from Opaline token names to `TextMate` scope selectors.
const SCOPE_MAPPINGS: &[(&str, &str)] = &[
    ("code.keyword", "keyword"),
    ("code.string", "string"),
    ("code.comment", "comment"),
    ("code.function", "entity.name.function, support.function"),
    ("code.number", "constant.numeric"),
    ("code.type", "entity.name.type, support.type"),
    ("code.hash", "constant.other"),
    ("code.path", "string.other.link"),
    ("code.line_number", "constant.numeric.line-number"),
    ("accent.primary", "variable"),
    ("accent.secondary", "storage.type, storage.modifier"),
    ("success", "markup.inserted"),
    ("error", "invalid, message.error"),
    ("warning", "markup.changed"),
    ("diff.added", "markup.inserted"),
    ("diff.removed", "markup.deleted"),
];

/// Convert an Opaline [`Theme`] to a [`syntect::highlighting::Theme`].
///
/// Maps theme metadata, token colors to `ThemeSettings`, and `code.*` tokens
/// to `TextMate` scope selectors for syntax highlighting.
pub fn to_syntect_theme(theme: &Theme) -> SyntectTheme {
    let settings = build_settings(theme);
    let scopes = build_scopes(theme);

    SyntectTheme {
        name: Some(theme.meta.name.clone()),
        author: theme.meta.author.clone(),
        settings,
        scopes,
    }
}

fn build_settings(theme: &Theme) -> ThemeSettings {
    ThemeSettings {
        foreground: theme.try_color("text.primary").map(Into::into),
        background: theme.try_color("bg.base").map(Into::into),
        caret: theme.try_color("accent.primary").map(Into::into),
        line_highlight: theme.try_color("bg.highlight").map(Into::into),
        selection: theme.try_color("bg.selection").map(Into::into),
        selection_foreground: theme.try_color("text.primary").map(Into::into),
        gutter: theme.try_color("bg.panel").map(Into::into),
        gutter_foreground: theme.try_color("text.dim").map(Into::into),
        find_highlight: theme.try_color("warning").map(Into::into),
        accent: theme.try_color("accent.primary").map(Into::into),
        guide: theme.try_color("border.unfocused").map(Into::into),
        active_guide: theme.try_color("border.focused").map(Into::into),
        brackets_foreground: theme.try_color("accent.secondary").map(Into::into),
        ..ThemeSettings::default()
    }
}

fn build_scopes(theme: &Theme) -> Vec<ThemeItem> {
    let mut items = Vec::new();

    for &(token, scope_str) in SCOPE_MAPPINGS {
        if let Some(color) = theme.try_color(token) {
            // Check if there's a corresponding style with modifiers
            let style_name = token
                .strip_prefix("code.")
                .unwrap_or(token);
            let font_style = theme
                .try_style(style_name)
                .map(|s| {
                    let mut fs = FontStyle::empty();
                    if s.bold {
                        fs |= FontStyle::BOLD;
                    }
                    if s.italic {
                        fs |= FontStyle::ITALIC;
                    }
                    if s.underline {
                        fs |= FontStyle::UNDERLINE;
                    }
                    fs
                })
                .filter(|fs| !fs.is_empty());

            if let Ok(scope) = scope_str.parse::<ScopeSelectors>() {
                items.push(ThemeItem {
                    scope,
                    style: StyleModifier {
                        foreground: Some(color.into()),
                        background: None,
                        font_style,
                    },
                });
            }
        }
    }

    items
}
