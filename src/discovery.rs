use std::path::PathBuf;

/// Return directories to scan for user-installed theme TOML files.
///
/// Currently returns `~/.config/opaline/themes/` and the XDG config equivalent.
/// Applications can extend this with their own paths.
pub fn theme_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(config) = dirs::config_dir() {
        dirs.push(config.join("opaline").join("themes"));
    }

    dirs
}
