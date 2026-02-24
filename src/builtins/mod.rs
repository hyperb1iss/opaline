use crate::loader;
use crate::schema::ThemeVariant;
use crate::theme::Theme;

// ── Embedded TOML sources ────────────────────────────────────────────────

const SILKCIRCUIT_NEON_TOML: &str = include_str!("silkcircuit_neon.toml");

// ── Registry ─────────────────────────────────────────────────────────────

/// All known builtin theme IDs paired with their display names.
pub fn builtin_names() -> &'static [(&'static str, &'static str)] {
    &[("silkcircuit-neon", "SilkCircuit Neon")]
}

/// Load a builtin theme by its kebab-case ID.
///
/// Returns `None` if the name doesn't match any builtin.
/// Use `"default"` as an alias for `"silkcircuit-neon"`.
pub fn load_by_name(name: &str) -> Option<Theme> {
    match name {
        "silkcircuit-neon" | "default" => Some(silkcircuit_neon()),
        _ => None,
    }
}

// ── Individual loaders ───────────────────────────────────────────────────

/// Load the `SilkCircuit` Neon theme (the default).
pub fn silkcircuit_neon() -> Theme {
    loader::load_from_str(SILKCIRCUIT_NEON_TOML, None)
        .expect("builtin silkcircuit-neon theme must be valid TOML")
}

// ── Theme info ───────────────────────────────────────────────────────────

/// Metadata about a discoverable theme.
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    /// The kebab-case identifier used for loading.
    pub name: String,
    /// The human-readable display name from `[meta]`.
    pub display_name: String,
    /// Dark or light.
    pub variant: ThemeVariant,
    /// Theme author.
    pub author: String,
    /// Short description.
    pub description: String,
    /// Whether this theme is compiled into the binary.
    pub builtin: bool,
    /// Filesystem path for user-installed themes, `None` for builtins.
    pub path: Option<std::path::PathBuf>,
}

/// List all available themes: builtins first, then user-installed themes
/// from discovery paths.
pub fn list_available_themes() -> Vec<ThemeInfo> {
    let mut themes = Vec::new();

    // Builtins
    for &(id, _display) in builtin_names() {
        if let Some(theme) = load_by_name(id) {
            themes.push(ThemeInfo {
                name: id.to_string(),
                display_name: theme.meta.name.clone(),
                variant: theme.meta.variant,
                author: theme.meta.author.clone().unwrap_or_default(),
                description: theme.meta.description.clone().unwrap_or_default(),
                builtin: true,
                path: None,
            });
        }
    }

    // Discovery paths (when enabled)
    #[cfg(feature = "discovery")]
    {
        for dir in crate::discovery::theme_dirs() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|e| e == "toml") {
                        if let Ok(theme) = loader::load_from_file(&path) {
                            let id = path
                                .file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();
                            themes.push(ThemeInfo {
                                name: id,
                                display_name: theme.meta.name.clone(),
                                variant: theme.meta.variant,
                                author: theme.meta.author.clone().unwrap_or_default(),
                                description: theme
                                    .meta
                                    .description
                                    .clone()
                                    .unwrap_or_default(),
                                builtin: false,
                                path: Some(path),
                            });
                        }
                    }
                }
            }
        }
    }

    themes
}
