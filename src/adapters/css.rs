//! CSS adapter — generate stylesheets from Opaline themes.
//!
//! Produces CSS custom properties from theme tokens and CSS classes from
//! theme styles. Gradient stops become `linear-gradient()` values.
//!
//! ```rust,ignore
//! let theme = Theme::default();
//! let css = opaline::adapters::css::generate_stylesheet(&theme);
//! // :root {
//! //   --opaline-accent-primary: #cba6f7;
//! //   ...
//! // }
//! ```

use crate::theme::Theme;

/// Generate CSS custom properties from all theme tokens.
///
/// Token names are prefixed with `--opaline-` and dots/underscores become dashes:
/// `accent.primary` → `--opaline-accent-primary: #cba6f7;`
///
/// When the `gradients` feature is enabled, gradient stops are emitted as
/// `linear-gradient(to right, ...)` values.
pub fn generate_css_vars(theme: &Theme) -> String {
    let mut lines = Vec::new();
    lines.push(":root {".to_string());

    let mut names = theme.token_names();
    names.sort_unstable();

    for name in names {
        let color = theme.color(name);
        let var_name = css_ident(name);
        lines.push(format!("  --opaline-{var_name}: {};", color.to_hex()));
    }

    #[cfg(feature = "gradients")]
    {
        let mut grad_names = theme.gradient_names();
        grad_names.sort_unstable();

        for name in grad_names {
            if let Some(gradient) = theme.get_gradient(name) {
                let stops: Vec<String> = gradient.stops().iter().map(|c| c.to_hex()).collect();
                let var_name = css_ident(name);
                lines.push(format!(
                    "  --opaline-gradient-{var_name}: linear-gradient(to right, {});",
                    stops.join(", ")
                ));
            }
        }
    }

    lines.push("}".to_string());
    lines.join("\n")
}

/// Generate CSS classes from all theme styles.
///
/// Style names are prefixed with `.opaline-` and underscores become dashes:
/// `keyword` → `.opaline-keyword { color: #cba6f7; font-weight: bold; }`
pub fn generate_css_classes(theme: &Theme) -> String {
    let mut blocks = Vec::new();

    let mut names = theme.style_names();
    names.sort_unstable();

    for name in names {
        let style = theme.style(name);
        let class_name = css_ident(name);
        let mut props = Vec::new();

        if let Some(fg) = style.fg {
            props.push(format!("  color: {};", fg.to_hex()));
        }
        if let Some(bg) = style.bg {
            props.push(format!("  background-color: {};", bg.to_hex()));
        }
        if style.bold {
            props.push("  font-weight: bold;".to_string());
        }
        if style.dim {
            props.push("  opacity: 0.7;".to_string());
        }
        if style.italic {
            props.push("  font-style: italic;".to_string());
        }

        let mut decorations = Vec::new();
        if style.underline {
            decorations.push("underline");
        }
        if style.crossed_out {
            decorations.push("line-through");
        }
        if !decorations.is_empty() {
            props.push(format!("  text-decoration: {};", decorations.join(" ")));
        }

        if style.hidden {
            props.push("  visibility: hidden;".to_string());
        }

        if !props.is_empty() {
            blocks.push(format!(
                ".opaline-{class_name} {{\n{}\n}}",
                props.join("\n")
            ));
        }
    }

    blocks.join("\n\n")
}

/// Generate a complete CSS stylesheet with custom properties and classes.
pub fn generate_stylesheet(theme: &Theme) -> String {
    let vars = generate_css_vars(theme);
    let classes = generate_css_classes(theme);

    if classes.is_empty() {
        vars
    } else {
        format!("{vars}\n\n{classes}")
    }
}

/// Normalize a token/style name into a valid CSS identifier fragment.
/// Dots and underscores become dashes, and the remaining unsafe characters
/// are escaped so arbitrary runtime names still produce valid selectors.
fn css_ident(name: &str) -> String {
    let normalized = name.replace(['.', '_'], "-");
    escape_css_ident_fragment(&normalized)
}

fn escape_css_ident_fragment(name: &str) -> String {
    if name.is_empty() {
        return "_".to_string();
    }

    let chars: Vec<char> = name.chars().collect();
    let mut escaped = String::with_capacity(name.len());

    for (idx, ch) in chars.iter().copied().enumerate() {
        let next_is_digit = chars.get(idx + 1).is_some_and(char::is_ascii_digit);
        let safe_ascii = ch.is_ascii_alphabetic() || ch == '_' || ch == '-';
        let safe_digit = idx > 0 && ch.is_ascii_digit();
        let needs_escape = !(safe_ascii || safe_digit)
            || (idx == 0 && ch.is_ascii_digit())
            || (idx == 0 && ch == '-' && next_is_digit);

        if needs_escape {
            push_css_escape(&mut escaped, ch);
        } else {
            escaped.push(ch);
        }
    }

    escaped
}

fn push_css_escape(output: &mut String, ch: char) {
    use std::fmt::Write as _;

    write!(output, "\\{:x} ", u32::from(ch)).expect("write to string");
}
