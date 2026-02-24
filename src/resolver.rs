use std::collections::HashMap;

use crate::color::OpalineColor;
use crate::error::OpalineError;
#[cfg(feature = "gradients")]
use crate::gradient::Gradient;
use crate::schema::{StyleDef, ThemeFile};
use crate::style::OpalineStyle;

/// Resolved theme data produced by the resolution pipeline.
#[derive(Debug, Clone)]
pub struct ResolvedTheme {
    pub palette: HashMap<String, OpalineColor>,
    pub tokens: HashMap<String, OpalineColor>,
    pub styles: HashMap<String, OpalineStyle>,
    #[cfg(feature = "gradients")]
    pub gradients: HashMap<String, Gradient>,
}

/// Resolve a parsed `ThemeFile` into concrete colors, styles, and gradients.
///
/// The resolution pipeline:
/// 1. **Palette**: Each value must be a literal hex string
/// 2. **Tokens**: Recursive resolution — references palette names, other tokens, or hex
/// 3. **Styles**: `fg`/`bg` resolved via tokens → palette → hex
/// 4. **Gradients**: Each stop resolved via tokens → palette → hex
pub fn resolve(theme_file: &ThemeFile) -> Result<ResolvedTheme, OpalineError> {
    let palette = resolve_palette(&theme_file.palette)?;
    let tokens = resolve_tokens(&theme_file.tokens, &palette)?;
    let styles = resolve_styles(&theme_file.styles, &palette, &tokens)?;
    #[cfg(feature = "gradients")]
    let gradients = resolve_gradients(&theme_file.gradients, &palette, &tokens)?;

    Ok(ResolvedTheme {
        palette,
        tokens,
        styles,
        #[cfg(feature = "gradients")]
        gradients,
    })
}

/// Pass 1: Every palette value must be a literal `#rrggbb` hex string.
fn resolve_palette(
    raw: &HashMap<String, String>,
) -> Result<HashMap<String, OpalineColor>, OpalineError> {
    let mut palette = HashMap::with_capacity(raw.len());
    for (name, hex) in raw {
        let color = OpalineColor::from_hex(hex).map_err(|source| OpalineError::InvalidColor {
            token: name.clone(),
            source,
        })?;
        palette.insert(name.clone(), color);
    }
    Ok(palette)
}

/// Pass 2: Recursively resolve tokens. A token value can be:
/// - A `#rrggbb` hex literal
/// - A palette name
/// - Another token name (chains allowed, cycles detected)
fn resolve_tokens(
    raw: &HashMap<String, String>,
    palette: &HashMap<String, OpalineColor>,
) -> Result<HashMap<String, OpalineColor>, OpalineError> {
    let mut resolved: HashMap<String, OpalineColor> = HashMap::with_capacity(raw.len());

    for name in raw.keys() {
        if !resolved.contains_key(name) {
            let mut chain = Vec::new();
            resolve_token(name, raw, palette, &mut resolved, &mut chain)?;
        }
    }

    Ok(resolved)
}

fn resolve_token(
    name: &str,
    raw: &HashMap<String, String>,
    palette: &HashMap<String, OpalineColor>,
    resolved: &mut HashMap<String, OpalineColor>,
    chain: &mut Vec<String>,
) -> Result<OpalineColor, OpalineError> {
    // Already resolved in a previous pass
    if let Some(&color) = resolved.get(name) {
        return Ok(color);
    }

    // Cycle detection — Vec preserves traversal order for readable error messages
    if chain.contains(&name.to_string()) {
        chain.push(name.to_string()); // close the cycle in the output
        return Err(OpalineError::CircularReference {
            token: name.to_string(),
            chain: chain.clone(),
        });
    }
    chain.push(name.to_string());

    let Some(value) = raw.get(name) else {
        // Token references a name not defined anywhere
        return Err(OpalineError::UnresolvedToken {
            token: name.to_string(),
            reference: name.to_string(),
        });
    };

    let color = if value.starts_with('#') {
        // Direct hex literal
        OpalineColor::from_hex(value).map_err(|source| OpalineError::InvalidColor {
            token: name.to_string(),
            source,
        })?
    } else if let Some(&palette_color) = palette.get(value.as_str()) {
        // Palette reference
        palette_color
    } else if raw.contains_key(value.as_str()) {
        // Token-to-token reference — recurse
        resolve_token(value, raw, palette, resolved, chain)?
    } else {
        // Unresolvable reference — report error so theme authors get feedback
        return Err(OpalineError::UnresolvedToken {
            token: name.to_string(),
            reference: value.clone(),
        });
    };

    resolved.insert(name.to_string(), color);
    Ok(color)
}

/// Resolve a color reference from the combined token + palette namespace.
/// Lookup order: hex literal → tokens → palette → `None`.
fn resolve_color_ref(
    reference: &str,
    palette: &HashMap<String, OpalineColor>,
    tokens: &HashMap<String, OpalineColor>,
) -> Option<OpalineColor> {
    if reference.starts_with('#') {
        OpalineColor::from_hex(reference).ok()
    } else if let Some(&color) = tokens.get(reference) {
        Some(color)
    } else {
        palette.get(reference).copied()
    }
}

/// Pass 3: Resolve style definitions into concrete `OpalineStyle` values.
fn resolve_styles(
    raw: &HashMap<String, StyleDef>,
    palette: &HashMap<String, OpalineColor>,
    tokens: &HashMap<String, OpalineColor>,
) -> Result<HashMap<String, OpalineStyle>, OpalineError> {
    let mut styles = HashMap::with_capacity(raw.len());

    for (name, def) in raw {
        let fg = def
            .fg
            .as_ref()
            .map(|r| {
                resolve_color_ref(r, palette, tokens).ok_or_else(|| OpalineError::UnresolvedToken {
                    token: format!("{name}.fg"),
                    reference: r.clone(),
                })
            })
            .transpose()?;
        let bg = def
            .bg
            .as_ref()
            .map(|r| {
                resolve_color_ref(r, palette, tokens).ok_or_else(|| OpalineError::UnresolvedToken {
                    token: format!("{name}.bg"),
                    reference: r.clone(),
                })
            })
            .transpose()?;

        styles.insert(
            name.clone(),
            OpalineStyle {
                fg,
                bg,
                bold: def.bold,
                italic: def.italic,
                underline: def.underline,
                dim: def.dim,
            },
        );
    }

    Ok(styles)
}

/// Pass 4: Resolve gradient stop names into color vectors.
#[cfg(feature = "gradients")]
fn resolve_gradients(
    raw: &HashMap<String, Vec<String>>,
    palette: &HashMap<String, OpalineColor>,
    tokens: &HashMap<String, OpalineColor>,
) -> Result<HashMap<String, Gradient>, OpalineError> {
    let mut gradients = HashMap::with_capacity(raw.len());

    for (name, stops) in raw {
        let mut colors = Vec::with_capacity(stops.len());
        for (i, stop) in stops.iter().enumerate() {
            let color =
                resolve_color_ref(stop, palette, tokens).ok_or_else(|| OpalineError::UnresolvedToken {
                    token: format!("{name}[{i}]"),
                    reference: stop.clone(),
                })?;
            colors.push(color);
        }

        if !colors.is_empty() {
            gradients.insert(name.clone(), Gradient::new(colors));
        }
    }

    Ok(gradients)
}
