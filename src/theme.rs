use std::collections::HashMap;

use crate::color::OpalineColor;
#[cfg(feature = "gradients")]
use crate::gradient::Gradient;
use crate::resolver::ResolvedTheme;
use crate::schema::{ThemeMeta, ThemeVariant};
use crate::style::OpalineStyle;

/// A fully resolved theme ready for use.
///
/// Contains resolved palette colors, semantic tokens, composed styles, and
/// gradients. Access colors by token name with [`color()`](Self::color),
/// styles by name with [`style()`](Self::style), and gradients with
/// [`gradient()`](Self::gradient).
#[derive(Debug, Clone)]
pub struct Theme {
    pub meta: ThemeMeta,
    palette: HashMap<String, OpalineColor>,
    tokens: HashMap<String, OpalineColor>,
    styles: HashMap<String, OpalineStyle>,
    #[cfg(feature = "gradients")]
    gradients: HashMap<String, Gradient>,
}

impl Theme {
    /// Construct a `Theme` from resolved data. Called by the loader.
    pub(crate) fn from_resolved(meta: ThemeMeta, resolved: ResolvedTheme) -> Self {
        Self {
            meta,
            palette: resolved.palette,
            tokens: resolved.tokens,
            styles: resolved.styles,
            #[cfg(feature = "gradients")]
            gradients: resolved.gradients,
        }
    }

    // ── Color access ─────────────────────────────────────────────────────

    /// Look up a color by token name, falling back to palette, then `FALLBACK`.
    pub fn color(&self, token: &str) -> OpalineColor {
        self.tokens
            .get(token)
            .or_else(|| self.palette.get(token))
            .copied()
            .unwrap_or(OpalineColor::FALLBACK)
    }

    /// Check whether a token or palette name exists.
    pub fn has_token(&self, name: &str) -> bool {
        self.tokens.contains_key(name) || self.palette.contains_key(name)
    }

    /// All token names defined in this theme.
    pub fn token_names(&self) -> Vec<&str> {
        self.tokens.keys().map(String::as_str).collect()
    }

    // ── Style access ─────────────────────────────────────────────────────

    /// Look up a style by name, returning `Default` if missing.
    pub fn style(&self, name: &str) -> OpalineStyle {
        self.styles
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check whether a named style exists.
    pub fn has_style(&self, name: &str) -> bool {
        self.styles.contains_key(name)
    }

    /// All style names defined in this theme.
    pub fn style_names(&self) -> Vec<&str> {
        self.styles.keys().map(String::as_str).collect()
    }

    // ── Gradient access ──────────────────────────────────────────────────

    /// Sample a named gradient at position `t`. Returns `FALLBACK` if the
    /// gradient doesn't exist or the feature is disabled.
    #[cfg(feature = "gradients")]
    pub fn gradient(&self, name: &str, t: f32) -> OpalineColor {
        self.gradients
            .get(name)
            .map_or(OpalineColor::FALLBACK, |g| g.at(t))
    }

    /// Get a reference to a named gradient for manual sampling.
    #[cfg(feature = "gradients")]
    pub fn get_gradient(&self, name: &str) -> Option<&Gradient> {
        self.gradients.get(name)
    }

    /// Check whether a named gradient exists.
    #[cfg(feature = "gradients")]
    pub fn has_gradient(&self, name: &str) -> bool {
        self.gradients.contains_key(name)
    }

    /// All gradient names defined in this theme.
    #[cfg(feature = "gradients")]
    pub fn gradient_names(&self) -> Vec<&str> {
        self.gradients.keys().map(String::as_str).collect()
    }

    // ── Variant helpers ──────────────────────────────────────────────────

    /// Whether this is a dark theme.
    pub fn is_dark(&self) -> bool {
        self.meta.variant == ThemeVariant::Dark
    }

    /// Whether this is a light theme.
    pub fn is_light(&self) -> bool {
        self.meta.variant == ThemeVariant::Light
    }
}

impl Default for Theme {
    fn default() -> Self {
        #[cfg(feature = "builtin-themes")]
        {
            crate::builtins::load_by_name("silkcircuit-neon")
                .expect("default builtin theme must be valid")
        }

        #[cfg(not(feature = "builtin-themes"))]
        {
            Self {
                meta: ThemeMeta {
                    name: "Fallback".to_string(),
                    author: None,
                    variant: ThemeVariant::Dark,
                    version: None,
                    description: None,
                },
                palette: HashMap::new(),
                tokens: HashMap::new(),
                styles: HashMap::new(),
                #[cfg(feature = "gradients")]
                gradients: HashMap::new(),
            }
        }
    }
}

// ── Global state (behind `global-state` feature) ─────────────────────────

#[cfg(feature = "global-state")]
mod global {
    use std::sync::{Arc, LazyLock};

    use parking_lot::RwLock;

    use super::Theme;
    use crate::error::OpalineError;

    static ACTIVE_THEME: LazyLock<RwLock<Arc<Theme>>> =
        LazyLock::new(|| RwLock::new(Arc::new(Theme::default())));

    /// Get a snapshot of the currently active global theme.
    pub fn current() -> Arc<Theme> {
        ACTIVE_THEME.read().clone()
    }

    /// Replace the active global theme.
    pub fn set_theme(theme: Theme) {
        *ACTIVE_THEME.write() = Arc::new(theme);
    }

    /// Load a builtin theme by name and set it as the active global theme.
    #[cfg(feature = "builtin-themes")]
    pub fn load_theme_by_name(name: &str) -> Result<(), OpalineError> {
        let theme = crate::builtins::load_by_name(name)
            .ok_or_else(|| OpalineError::ThemeNotFound {
                name: name.to_string(),
            })?;
        set_theme(theme);
        Ok(())
    }

    /// Load a theme from a file and set it as the active global theme.
    pub fn load_theme(path: &std::path::Path) -> Result<(), OpalineError> {
        let theme = crate::loader::load_from_file(path)?;
        set_theme(theme);
        Ok(())
    }
}

#[cfg(feature = "global-state")]
pub use global::{current, load_theme, set_theme};

#[cfg(all(feature = "global-state", feature = "builtin-themes"))]
pub use global::load_theme_by_name;
