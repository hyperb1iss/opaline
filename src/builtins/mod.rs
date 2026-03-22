use crate::loader;
use crate::schema::ThemeVariant;
use crate::theme::Theme;

// Pull in auto-generated constants, registry, and loader from build.rs
include!(concat!(env!("OUT_DIR"), "/builtins_generated.rs"));

// ── Public API ───────────────────────────────────────────────────────────

/// Total number of builtin themes (auto-discovered at compile time).
pub const BUILTIN_COUNT: usize = GENERATED_BUILTIN_COUNT;

/// All known builtin theme IDs paired with their display names.
///
/// Auto-discovered from `src/builtins/*.toml` at compile time.
/// IDs are kebab-case derived from filenames; display names are
/// extracted from each theme's `[meta].name` field.
pub fn builtin_names() -> &'static [(&'static str, &'static str)] {
    GENERATED_BUILTIN_NAMES
}

/// Load a builtin theme by its kebab-case ID.
///
/// Returns `None` if the name doesn't match any builtin.
/// Use `"default"` as an alias for `"silkcircuit-neon"`.
pub fn load_by_name(name: &str) -> Option<Theme> {
    let name = if name == "default" {
        "silkcircuit-neon"
    } else {
        name
    };
    let toml_str = generated_load_toml(name)?;
    Some(
        loader::load_from_str(toml_str, None)
            .unwrap_or_else(|e| panic!("builtin theme '{name}' must be valid TOML: {e}")),
    )
}

/// Load the `SilkCircuit` Neon theme (the default).
pub fn silkcircuit_neon() -> Theme {
    load_by_name("silkcircuit-neon").expect("default builtin must exist")
}

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

impl ThemeInfo {
    /// Load the theme represented by this metadata.
    ///
    /// File-backed themes prefer their filesystem path. Builtins fall back to
    /// the compile-time registry.
    #[must_use]
    pub fn load(&self) -> Option<Theme> {
        self.path
            .as_ref()
            .and_then(|path| loader::load_from_file(path).ok())
            .or_else(|| load_by_name(&self.name))
    }
}

impl std::fmt::Display for ThemeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.display_name, self.variant)?;
        if !self.author.is_empty() {
            write!(f, " by {}", self.author)?;
        }
        Ok(())
    }
}

/// List all available themes: builtins plus user-installed themes from
/// discovery paths. File-backed themes replace builtin entries when ids
/// collide.
pub fn list_available_themes() -> Vec<ThemeInfo> {
    #[cfg(feature = "discovery")]
    {
        list_available_themes_in_dirs(crate::discovery::theme_dirs())
    }

    #[cfg(not(feature = "discovery"))]
    {
        builtin_theme_infos()
    }
}

/// List all available themes for a specific application.
///
/// This includes the base Opaline discovery directory plus the app-specific
/// `~/.config/<app_name>/themes/` directory. When ids collide, file-backed
/// themes replace builtin entries.
#[cfg(feature = "discovery")]
pub fn list_available_themes_for_app(app_name: &str) -> Vec<ThemeInfo> {
    list_available_themes_in_dirs(crate::discovery::app_theme_dirs(app_name))
}

/// List all available themes using an explicit set of discovery directories.
///
/// Builtins are always included. File-backed themes replace builtin entries
/// when ids collide. This is useful for applications that manage their own
/// theme search paths or for tests that need deterministic discovery.
#[cfg(feature = "discovery")]
pub fn list_available_themes_in_dirs<I, P>(dirs: I) -> Vec<ThemeInfo>
where
    I: IntoIterator<Item = P>,
    P: Into<std::path::PathBuf>,
{
    let mut themes = builtin_theme_infos();
    scan_theme_dirs(&mut themes, dirs);
    themes
}

fn builtin_theme_infos() -> Vec<ThemeInfo> {
    let mut themes = Vec::new();

    for &(id, _) in builtin_names() {
        if let Some(theme) = load_by_name(id) {
            push_or_replace_theme(
                &mut themes,
                ThemeInfo {
                    name: id.to_string(),
                    display_name: theme.meta.name.clone(),
                    variant: theme.meta.variant,
                    author: theme.meta.author.clone().unwrap_or_default(),
                    description: theme.meta.description.clone().unwrap_or_default(),
                    builtin: true,
                    path: None,
                },
            );
        }
    }

    themes
}

fn push_or_replace_theme(themes: &mut Vec<ThemeInfo>, info: ThemeInfo) {
    if let Some(existing) = themes.iter_mut().find(|theme| theme.name == info.name) {
        *existing = info;
    } else {
        themes.push(info);
    }
}

#[cfg(feature = "discovery")]
fn scan_theme_dirs<I, P>(themes: &mut Vec<ThemeInfo>, dirs: I)
where
    I: IntoIterator<Item = P>,
    P: Into<std::path::PathBuf>,
{
    for dir in dirs.into_iter().map(Into::into) {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml")
                    && let Ok(theme) = loader::load_from_file(&path)
                {
                    let id = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();
                    push_or_replace_theme(
                        themes,
                        ThemeInfo {
                            name: id,
                            display_name: theme.meta.name.clone(),
                            variant: theme.meta.variant,
                            author: theme.meta.author.clone().unwrap_or_default(),
                            description: theme.meta.description.clone().unwrap_or_default(),
                            builtin: false,
                            path: Some(path),
                        },
                    );
                }
            }
        }
    }
}
