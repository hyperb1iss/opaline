#![cfg(feature = "syntect")]

use syntect::highlighting::{Color, FontStyle, StyleModifier};

use opaline::{OpalineColor, OpalineStyle, Theme};

#[test]
fn color_to_syntect() {
    let c = OpalineColor::new(225, 53, 255);
    let sc: Color = c.into();
    assert_eq!(sc.r, 225);
    assert_eq!(sc.g, 53);
    assert_eq!(sc.b, 255);
    assert_eq!(sc.a, 255);
}

#[test]
fn color_ref_to_syntect() {
    let c = OpalineColor::new(128, 255, 234);
    let sc: Color = (&c).into();
    assert_eq!(sc.r, 128);
    assert_eq!(sc.g, 255);
    assert_eq!(sc.b, 234);
    assert_eq!(sc.a, 255);
}

#[test]
fn style_to_modifier_fg_only() {
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255));
    let m: StyleModifier = s.into();
    assert_eq!(m.foreground, Some(Color { r: 225, g: 53, b: 255, a: 255 }));
    assert_eq!(m.background, None);
    assert_eq!(m.font_style, None);
}

#[test]
fn style_to_modifier_with_modifiers() {
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold().italic();
    let m: StyleModifier = s.into();
    let fs = m.font_style.expect("should have font style");
    assert!(fs.contains(FontStyle::BOLD));
    assert!(fs.contains(FontStyle::ITALIC));
    assert!(!fs.contains(FontStyle::UNDERLINE));
}

#[test]
fn style_to_modifier_underline() {
    let s = OpalineStyle::new().underline();
    let m: StyleModifier = s.into();
    let fs = m.font_style.expect("should have font style");
    assert!(fs.contains(FontStyle::UNDERLINE));
}

#[test]
fn style_ref_to_modifier() {
    let s = OpalineStyle::fg(OpalineColor::new(80, 250, 123)).bold();
    let m: StyleModifier = (&s).into();
    assert!(m.foreground.is_some());
    assert!(m.font_style.expect("font style").contains(FontStyle::BOLD));
}

#[test]
fn to_syntect_theme_metadata() {
    let theme = Theme::builder("SilkCircuit Neon")
        .author("hyperb1iss")
        .token("text.primary", OpalineColor::new(205, 214, 244))
        .token("bg.base", OpalineColor::new(30, 30, 46))
        .build();

    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    assert_eq!(st.name, Some("SilkCircuit Neon".to_string()));
    assert_eq!(st.author, Some("hyperb1iss".to_string()));
}

#[test]
fn to_syntect_theme_settings() {
    let theme = Theme::builder("Test")
        .token("text.primary", OpalineColor::new(205, 214, 244))
        .token("bg.base", OpalineColor::new(30, 30, 46))
        .token("accent.primary", OpalineColor::new(203, 166, 247))
        .token("bg.highlight", OpalineColor::new(49, 50, 68))
        .build();

    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    assert_eq!(st.settings.foreground, Some(Color { r: 205, g: 214, b: 244, a: 255 }));
    assert_eq!(st.settings.background, Some(Color { r: 30, g: 30, b: 46, a: 255 }));
    assert_eq!(st.settings.caret, Some(Color { r: 203, g: 166, b: 247, a: 255 }));
    assert_eq!(st.settings.line_highlight, Some(Color { r: 49, g: 50, b: 68, a: 255 }));
}

#[test]
fn to_syntect_theme_scopes() {
    let theme = Theme::builder("Test")
        .token("code.keyword", OpalineColor::new(203, 166, 247))
        .token("code.string", OpalineColor::new(166, 227, 161))
        .token("code.comment", OpalineColor::new(108, 112, 134))
        .build();

    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    assert!(!st.scopes.is_empty());
    // Should have scopes for keyword, string, comment
    assert!(st.scopes.len() >= 3);
}

#[test]
fn to_syntect_theme_scope_with_style_modifiers() {
    let theme = Theme::builder("Test")
        .token("code.keyword", OpalineColor::new(203, 166, 247))
        .style("keyword", OpalineStyle::fg(OpalineColor::new(203, 166, 247)).bold())
        .build();

    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    let keyword_scope = st.scopes.iter()
        .find(|item| {
            item.style.foreground == Some(Color { r: 203, g: 166, b: 247, a: 255 })
        })
        .expect("should have keyword scope");

    assert!(keyword_scope.style.font_style
        .expect("should have font style")
        .contains(FontStyle::BOLD));
}

#[test]
fn to_syntect_theme_missing_tokens_skipped() {
    let theme = Theme::builder("Minimal").build();
    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    assert!(st.scopes.is_empty());
    assert!(st.settings.foreground.is_none());
}

#[cfg(feature = "builtin-themes")]
#[test]
fn builtin_theme_converts_to_syntect() {
    let theme = Theme::default();
    let st = opaline::adapters::syntect::to_syntect_theme(&theme);
    assert!(st.name.is_some());
    assert!(st.settings.foreground.is_some());
    assert!(st.settings.background.is_some());
    assert!(!st.scopes.is_empty());
}
