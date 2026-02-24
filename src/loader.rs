use std::path::Path;

use crate::error::OpalineError;
use crate::resolver;
use crate::schema::ThemeFile;
use crate::theme::Theme;

/// Load a theme from a TOML string.
///
/// The optional `path` is stored for error diagnostics only.
pub fn load_from_str(toml_str: &str, path: Option<&Path>) -> Result<Theme, OpalineError> {
    let theme_file: ThemeFile = toml::from_str(toml_str).map_err(|source| OpalineError::Parse {
        path: path.map(Path::to_path_buf),
        source,
    })?;

    let resolved = resolver::resolve(&theme_file)?;
    Ok(Theme::from_resolved(theme_file.meta, resolved))
}

/// Load a theme from a TOML file on disk.
pub fn load_from_file(path: impl AsRef<Path>) -> Result<Theme, OpalineError> {
    let path = path.as_ref();
    let contents = std::fs::read_to_string(path).map_err(|source| OpalineError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    load_from_str(&contents, Some(path))
}
