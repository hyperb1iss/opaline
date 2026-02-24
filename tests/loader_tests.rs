use opaline::error::OpalineError;
use opaline::loader;
use opaline::{OpalineColor, OpalineStyle};
use pretty_assertions::assert_eq;

const MINIMAL_TOML: &str = r##"
[meta]
name = "Minimal"
variant = "dark"

[palette]
red = "#ff0000"
blue = "#0000ff"

[tokens]
"accent.primary" = "red"
"accent.secondary" = "blue"

[styles]
keyword = { fg = "accent.primary", bold = true }

[gradients]
primary = ["red", "blue"]
"##;

#[test]
fn load_minimal_theme_from_string() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(theme.meta.name, "Minimal");
    assert!(theme.is_dark());
}

#[test]
fn loaded_theme_resolves_tokens() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(
        theme.color("accent.primary"),
        OpalineColor::new(255, 0, 0)
    );
    assert_eq!(
        theme.color("accent.secondary"),
        OpalineColor::new(0, 0, 255)
    );
}

#[test]
fn loaded_theme_resolves_styles() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    let style = theme.style("keyword");
    assert_eq!(
        style,
        OpalineStyle {
            fg: Some(OpalineColor::new(255, 0, 0)),
            bg: None,
            bold: true,
            italic: false,
            underline: false,
            dim: false,
        }
    );
}

#[test]
fn loaded_theme_resolves_gradients() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(
        theme.gradient("primary", 0.0),
        OpalineColor::new(255, 0, 0)
    );
    assert_eq!(
        theme.gradient("primary", 1.0),
        OpalineColor::new(0, 0, 255)
    );
}

#[test]
fn missing_token_returns_fallback() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(theme.color("nonexistent"), OpalineColor::FALLBACK);
}

#[test]
fn missing_style_returns_default() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(theme.style("nonexistent"), OpalineStyle::default());
}

#[test]
fn missing_gradient_returns_fallback() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert_eq!(
        theme.gradient("nonexistent", 0.5),
        OpalineColor::FALLBACK
    );
}

#[test]
fn has_token_checks() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert!(theme.has_token("accent.primary"));
    assert!(!theme.has_token("nonexistent"));
}

#[test]
fn has_style_checks() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert!(theme.has_style("keyword"));
    assert!(!theme.has_style("nonexistent"));
}

#[test]
fn has_gradient_checks() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    assert!(theme.has_gradient("primary"));
    assert!(!theme.has_gradient("nonexistent"));
}

#[test]
fn invalid_toml_returns_parse_error() {
    let err = loader::load_from_str("this is not toml {{{", None).expect_err("should fail");
    assert!(matches!(err, OpalineError::Parse { .. }));
}

#[test]
fn load_from_file_nonexistent_returns_io_error() {
    let err = loader::load_from_file(std::path::Path::new("/tmp/opaline_nonexistent.toml"))
        .expect_err("should fail");
    assert!(matches!(err, OpalineError::Io { .. }));
}

#[test]
fn theme_token_names() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    let names = theme.token_names();
    assert!(names.contains(&"accent.primary"));
    assert!(names.contains(&"accent.secondary"));
}

#[test]
fn theme_style_names() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    let names = theme.style_names();
    assert!(names.contains(&"keyword"));
}

#[test]
fn theme_gradient_names() {
    let theme = loader::load_from_str(MINIMAL_TOML, None).expect("valid TOML");
    let names = theme.gradient_names();
    assert!(names.contains(&"primary"));
}

#[test]
fn light_theme_variant() {
    let toml = r#"
[meta]
name = "Light Test"
variant = "light"
"#;
    let theme = loader::load_from_str(toml, None).expect("valid TOML");
    assert!(theme.is_light());
    assert!(!theme.is_dark());
}
