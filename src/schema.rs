use std::collections::HashMap;

use serde::Deserialize;

/// Top-level structure of a `.toml` theme file.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ThemeFile {
    pub meta: ThemeMeta,

    /// Raw hex color definitions (e.g. `purple_500 = "#e135ff"`).
    #[serde(default)]
    pub palette: HashMap<String, String>,

    /// Semantic token assignments referencing palette names, other tokens, or hex.
    #[serde(default)]
    pub tokens: HashMap<String, String>,

    /// Composed styles with fg/bg references and modifiers.
    #[serde(default)]
    pub styles: HashMap<String, StyleDef>,

    /// Named gradients as arrays of color references.
    #[serde(default)]
    pub gradients: HashMap<String, Vec<String>>,
}

/// Theme metadata from the `[meta]` section.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct ThemeMeta {
    pub name: String,

    #[serde(default)]
    pub author: Option<String>,

    #[serde(default)]
    pub variant: ThemeVariant,

    #[serde(default)]
    pub version: Option<String>,

    #[serde(default)]
    pub description: Option<String>,
}

/// Whether a theme is designed for dark or light backgrounds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    #[default]
    Dark,
    Light,
}

/// Style definition as it appears in a TOML `[styles]` section.
///
/// Color references (`fg`, `bg`) are resolved against the token and palette maps
/// during theme loading.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Default, Deserialize, serde::Serialize)]
pub struct StyleDef {
    pub fg: Option<String>,
    pub bg: Option<String>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub dim: bool,
}
