use std::path::PathBuf;

use crate::color::ColorParseError;

/// All errors that can occur during theme loading and resolution.
#[derive(Debug, thiserror::Error)]
pub enum OpalineError {
    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("TOML parse error{}: {source}", path.as_ref().map(|p| format!(" in {}", p.display())).unwrap_or_default())]
    Parse {
        path: Option<PathBuf>,
        #[source]
        source: toml::de::Error,
    },

    #[error("invalid color for token '{token}': {source}")]
    InvalidColor {
        token: String,
        #[source]
        source: ColorParseError,
    },

    #[error("circular token reference '{token}': {}", chain.join(" \u{2192} "))]
    CircularReference { token: String, chain: Vec<String> },

    #[error("unresolved token '{token}' references '{reference}'")]
    UnresolvedToken { token: String, reference: String },

    #[error("missing required section: {section}")]
    MissingSection { section: String },

    #[error("theme not found: {name}")]
    ThemeNotFound { name: String },

    #[error("gradient must have at least one color stop")]
    EmptyGradient,
}
