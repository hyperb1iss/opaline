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
    /// Construct a `Theme` from pre-resolved data.
    ///
    /// For most use cases, prefer [`load_from_str`](crate::loader::load_from_str)
    /// or [`load_from_file`](crate::loader::load_from_file). Use this when you
    /// have already run the resolution pipeline yourself.
    pub fn from_resolved(meta: ThemeMeta, resolved: ResolvedTheme) -> Self {
        Self {
            meta,
            palette: resolved.palette,
            tokens: resolved.tokens,
            styles: resolved.styles,
            #[cfg(feature = "gradients")]
            gradients: resolved.gradients,
        }
    }

    /// Start building a theme programmatically.
    ///
    /// See [`ThemeBuilder`] for the full builder API.
    pub fn builder(name: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder::new(name)
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

    /// Strict color lookup — returns `None` if the token doesn't exist.
    pub fn try_color(&self, token: &str) -> Option<OpalineColor> {
        self.tokens
            .get(token)
            .or_else(|| self.palette.get(token))
            .copied()
    }

    /// Check whether a token or palette name exists.
    pub fn has_token(&self, name: &str) -> bool {
        self.tokens.contains_key(name) || self.palette.contains_key(name)
    }

    /// All token names defined in this theme.
    pub fn token_names(&self) -> Vec<&str> {
        self.tokens.keys().map(String::as_str).collect()
    }

    /// All palette color names defined in this theme.
    pub fn palette_names(&self) -> Vec<&str> {
        self.palette.keys().map(String::as_str).collect()
    }

    // ── Style access ─────────────────────────────────────────────────────

    /// Look up a style by name, returning `Default` if missing.
    pub fn style(&self, name: &str) -> OpalineStyle {
        self.styles.get(name).cloned().unwrap_or_default()
    }

    /// Strict style lookup — returns `None` if the style doesn't exist.
    pub fn try_style(&self, name: &str) -> Option<&OpalineStyle> {
        self.styles.get(name)
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

    /// Strict gradient sampling — returns `None` if the gradient doesn't exist.
    #[cfg(feature = "gradients")]
    pub fn try_gradient(&self, name: &str, t: f32) -> Option<OpalineColor> {
        self.gradients.get(name).map(|g| g.at(t))
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

    // ── Token injection ───────────────────────────────────────────────

    /// Register a token only if the theme doesn't already define it.
    ///
    /// Use this for app-level derived tokens — TOML-defined values take
    /// priority so theme authors can override derivations.
    pub fn register_default_token(&mut self, name: impl Into<String>, color: OpalineColor) {
        let key = name.into();
        self.tokens.entry(key).or_insert(color);
    }

    /// Register a token, overwriting any existing value.
    pub fn register_token(&mut self, name: impl Into<String>, color: OpalineColor) {
        self.tokens.insert(name.into(), color);
    }

    /// Register a style only if the theme doesn't already define it.
    pub fn register_default_style(&mut self, name: impl Into<String>, style: OpalineStyle) {
        let key = name.into();
        self.styles.entry(key).or_insert(style);
    }

    /// Register a style, overwriting any existing value.
    pub fn register_style(&mut self, name: impl Into<String>, style: OpalineStyle) {
        self.styles.insert(name.into(), style);
    }
}

// ── Builder ─────────────────────────────────────────────────────────

/// Programmatic theme construction without TOML.
///
/// ```rust
/// use opaline::{Theme, OpalineColor, OpalineStyle};
///
/// let theme = Theme::builder("My Theme")
///     .token("accent.primary", OpalineColor::new(225, 53, 255))
///     .token("bg.base", OpalineColor::new(18, 18, 24))
///     .style("keyword", OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold())
///     .build();
///
/// assert_eq!(theme.meta.name, "My Theme");
/// ```
pub struct ThemeBuilder {
    meta: ThemeMeta,
    palette: HashMap<String, OpalineColor>,
    tokens: HashMap<String, OpalineColor>,
    styles: HashMap<String, OpalineStyle>,
    #[cfg(feature = "gradients")]
    gradients: HashMap<String, Gradient>,
}

impl ThemeBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self {
            meta: ThemeMeta::new(name),
            palette: HashMap::new(),
            tokens: HashMap::new(),
            styles: HashMap::new(),
            #[cfg(feature = "gradients")]
            gradients: HashMap::new(),
        }
    }

    /// Set the theme author.
    #[must_use]
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.meta.author = Some(author.into());
        self
    }

    /// Set the theme variant (dark/light).
    #[must_use]
    pub fn variant(mut self, variant: ThemeVariant) -> Self {
        self.meta.variant = variant;
        self
    }

    /// Set the theme version.
    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.meta.version = Some(version.into());
        self
    }

    /// Set the theme description.
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.meta.description = Some(desc.into());
        self
    }

    /// Add a palette color.
    #[must_use]
    pub fn palette(mut self, name: impl Into<String>, color: OpalineColor) -> Self {
        self.palette.insert(name.into(), color);
        self
    }

    /// Add a semantic token.
    #[must_use]
    pub fn token(mut self, name: impl Into<String>, color: OpalineColor) -> Self {
        self.tokens.insert(name.into(), color);
        self
    }

    /// Add a composed style.
    #[must_use]
    pub fn style(mut self, name: impl Into<String>, style: OpalineStyle) -> Self {
        self.styles.insert(name.into(), style);
        self
    }

    /// Add a gradient.
    #[cfg(feature = "gradients")]
    #[must_use]
    pub fn gradient(mut self, name: impl Into<String>, gradient: Gradient) -> Self {
        self.gradients.insert(name.into(), gradient);
        self
    }

    /// Build the theme.
    #[must_use]
    pub fn build(self) -> Theme {
        Theme {
            meta: self.meta,
            palette: self.palette,
            tokens: self.tokens,
            styles: self.styles,
            #[cfg(feature = "gradients")]
            gradients: self.gradients,
        }
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

    /// Load a theme by name and set it as the active global theme.
    ///
    /// Searches builtins first, then discovery paths (if the `discovery`
    /// feature is enabled).
    #[cfg(feature = "builtin-themes")]
    pub fn load_theme_by_name(name: &str) -> Result<(), OpalineError> {
        // Check builtins first
        if let Some(theme) = crate::builtins::load_by_name(name) {
            set_theme(theme);
            return Ok(());
        }

        // Search discovery paths
        #[cfg(feature = "discovery")]
        {
            for dir in crate::discovery::theme_dirs() {
                let path = dir.join(format!("{name}.toml"));
                if path.exists() {
                    let theme = crate::loader::load_from_file(&path)?;
                    set_theme(theme);
                    return Ok(());
                }
            }
        }

        Err(OpalineError::ThemeNotFound {
            name: name.to_string(),
        })
    }

    /// Load a theme by name, run an app-level derivation callback, then
    /// set as the active global theme.
    ///
    /// The callback receives a mutable reference to the loaded theme,
    /// allowing apps to register derived tokens before it becomes active.
    #[cfg(feature = "builtin-themes")]
    pub fn load_theme_by_name_with<F>(name: &str, derive: F) -> Result<(), OpalineError>
    where
        F: FnOnce(&mut Theme),
    {
        // Check builtins first
        if let Some(mut theme) = crate::builtins::load_by_name(name) {
            derive(&mut theme);
            set_theme(theme);
            return Ok(());
        }

        // Search discovery paths
        #[cfg(feature = "discovery")]
        {
            for dir in crate::discovery::theme_dirs() {
                let path = dir.join(format!("{name}.toml"));
                if path.exists() {
                    let mut theme = crate::loader::load_from_file(&path)?;
                    derive(&mut theme);
                    set_theme(theme);
                    return Ok(());
                }
            }
        }

        Err(OpalineError::ThemeNotFound {
            name: name.to_string(),
        })
    }

    /// Load a theme by name, searching app-specific discovery paths too.
    ///
    /// Like [`load_theme_by_name`] but also searches
    /// `~/.config/<app_name>/themes/`.
    #[cfg(all(feature = "builtin-themes", feature = "discovery"))]
    pub fn load_theme_by_name_for_app(name: &str, app_name: &str) -> Result<(), OpalineError> {
        // Check builtins first
        if let Some(theme) = crate::builtins::load_by_name(name) {
            set_theme(theme);
            return Ok(());
        }

        // Search app-specific discovery paths
        for dir in crate::discovery::app_theme_dirs(app_name) {
            let path = dir.join(format!("{name}.toml"));
            if path.exists() {
                let theme = crate::loader::load_from_file(&path)?;
                set_theme(theme);
                return Ok(());
            }
        }

        Err(OpalineError::ThemeNotFound {
            name: name.to_string(),
        })
    }

    /// Like [`load_theme_by_name_for_app`] but with an app-level derivation callback.
    #[cfg(all(feature = "builtin-themes", feature = "discovery"))]
    pub fn load_theme_by_name_for_app_with<F>(
        name: &str,
        app_name: &str,
        derive: F,
    ) -> Result<(), OpalineError>
    where
        F: FnOnce(&mut Theme),
    {
        // Check builtins first
        if let Some(mut theme) = crate::builtins::load_by_name(name) {
            derive(&mut theme);
            set_theme(theme);
            return Ok(());
        }

        // Search app-specific discovery paths
        for dir in crate::discovery::app_theme_dirs(app_name) {
            let path = dir.join(format!("{name}.toml"));
            if path.exists() {
                let mut theme = crate::loader::load_from_file(&path)?;
                derive(&mut theme);
                set_theme(theme);
                return Ok(());
            }
        }

        Err(OpalineError::ThemeNotFound {
            name: name.to_string(),
        })
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

#[cfg(all(
    feature = "global-state",
    feature = "builtin-themes",
    feature = "discovery"
))]
pub use global::load_theme_by_name_for_app;

#[cfg(all(feature = "global-state", feature = "builtin-themes"))]
pub use global::load_theme_by_name_with;

#[cfg(all(
    feature = "global-state",
    feature = "builtin-themes",
    feature = "discovery"
))]
pub use global::load_theme_by_name_for_app_with;
