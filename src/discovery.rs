use std::path::PathBuf;

/// Return directories to scan for user-installed theme TOML files.
///
/// Includes the opaline-specific config directory plus any app-specific
/// directories registered via [`app_theme_dirs`].
pub fn theme_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(config) = dirs::config_dir() {
        dirs.push(config.join("opaline").join("themes"));
    }

    dirs
}

/// Return theme directories for a specific application.
///
/// Returns `~/.config/<app_name>/themes/` in addition to the base opaline
/// theme directories. This lets consumers like `git-iris` or `unifly-tui`
/// discover themes from their own config paths.
pub fn app_theme_dirs(app_name: &str) -> Vec<PathBuf> {
    let mut dirs = theme_dirs();

    if let Some(config) = dirs::config_dir() {
        dirs.push(config.join(app_name).join("themes"));
    }

    dirs
}
