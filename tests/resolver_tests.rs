use std::collections::HashMap;

use opaline::error::OpalineError;
use opaline::schema::{StyleDef, ThemeFile, ThemeMeta, ThemeVariant};
use opaline::{OpalineColor, OpalineStyle};

fn minimal_theme_file() -> ThemeFile {
    ThemeFile {
        meta: ThemeMeta {
            name: "Test".to_string(),
            author: None,
            variant: ThemeVariant::Dark,
            version: None,
            description: None,
        },
        palette: HashMap::new(),
        tokens: HashMap::new(),
        styles: HashMap::new(),
        gradients: HashMap::new(),
    }
}

#[test]
fn empty_theme_resolves() {
    let result = opaline::resolver::resolve(&minimal_theme_file());
    assert!(result.is_ok());
}

#[test]
fn palette_resolves_hex() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("red".to_string(), "#ff0000".to_string());

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    assert_eq!(
        resolved.palette.get("red"),
        Some(&OpalineColor::new(255, 0, 0))
    );
}

#[test]
fn palette_rejects_invalid_hex() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("bad".to_string(), "not-a-color".to_string());

    let err = opaline::resolver::resolve(&tf).expect_err("should fail");
    assert!(matches!(err, OpalineError::InvalidColor { .. }));
}

#[test]
fn token_references_palette() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("purple_500".to_string(), "#e135ff".to_string());
    tf.tokens
        .insert("accent.primary".to_string(), "purple_500".to_string());

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    assert_eq!(
        resolved.tokens.get("accent.primary"),
        Some(&OpalineColor::new(225, 53, 255))
    );
}

#[test]
fn token_references_hex_directly() {
    let mut tf = minimal_theme_file();
    tf.tokens
        .insert("direct".to_string(), "#ff6363".to_string());

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    assert_eq!(
        resolved.tokens.get("direct"),
        Some(&OpalineColor::new(255, 99, 99))
    );
}

#[test]
fn token_chains_to_another_token() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("green_400".to_string(), "#50fa7b".to_string());
    tf.tokens
        .insert("success".to_string(), "green_400".to_string());
    tf.tokens
        .insert("diff.added".to_string(), "success".to_string());

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    let expected = OpalineColor::new(80, 250, 123);
    assert_eq!(resolved.tokens.get("success"), Some(&expected));
    assert_eq!(resolved.tokens.get("diff.added"), Some(&expected));
}

#[test]
fn circular_reference_detected() {
    let mut tf = minimal_theme_file();
    tf.tokens.insert("a".to_string(), "b".to_string());
    tf.tokens.insert("b".to_string(), "a".to_string());

    let err = opaline::resolver::resolve(&tf).expect_err("should fail");
    assert!(matches!(err, OpalineError::CircularReference { .. }));
}

#[test]
fn unresolvable_token_returns_error() {
    let mut tf = minimal_theme_file();
    tf.tokens
        .insert("missing".to_string(), "nonexistent".to_string());

    let err = opaline::resolver::resolve(&tf).expect_err("should fail");
    assert!(matches!(err, OpalineError::UnresolvedToken { .. }));
}

#[test]
fn style_resolves_fg_bg() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("cyan_400".to_string(), "#80ffea".to_string());
    tf.tokens
        .insert("accent".to_string(), "cyan_400".to_string());
    tf.styles.insert(
        "highlight".to_string(),
        StyleDef {
            fg: Some("accent".to_string()),
            bg: Some("#1e1e28".to_string()),
            bold: true,
            ..StyleDef::default()
        },
    );

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    let style = resolved.styles.get("highlight").expect("style exists");

    assert_eq!(
        *style,
        OpalineStyle {
            fg: Some(OpalineColor::new(128, 255, 234)),
            bg: Some(OpalineColor::new(30, 30, 40)),
            bold: true,
            italic: false,
            underline: false,
            dim: false,
        }
    );
}

#[test]
fn gradient_resolves_stops() {
    let mut tf = minimal_theme_file();
    tf.palette
        .insert("red".to_string(), "#ff0000".to_string());
    tf.palette
        .insert("blue".to_string(), "#0000ff".to_string());
    tf.gradients
        .insert("test".to_string(), vec!["red".to_string(), "blue".to_string()]);

    let resolved = opaline::resolver::resolve(&tf).expect("resolves");
    let grad = resolved.gradients.get("test").expect("gradient exists");
    assert_eq!(grad.len(), 2);
    assert_eq!(grad.at(0.0), OpalineColor::new(255, 0, 0));
    assert_eq!(grad.at(1.0), OpalineColor::new(0, 0, 255));
}

#[test]
fn gradient_with_unresolvable_stops_returns_error() {
    let mut tf = minimal_theme_file();
    tf.gradients.insert(
        "broken".to_string(),
        vec!["nonexistent1".to_string(), "nonexistent2".to_string()],
    );

    let err = opaline::resolver::resolve(&tf).expect_err("should fail");
    assert!(matches!(err, OpalineError::UnresolvedToken { .. }));
}
