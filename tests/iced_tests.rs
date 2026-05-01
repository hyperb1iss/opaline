#![cfg(feature = "iced")]

use std::sync::Arc;

use iced_core::Color;
use iced_core::theme::{Base, Theme as IcedTheme};

use opaline::adapters::iced::{to_iced_custom, to_iced_extended, to_iced_palette};
use opaline::{OpalineColor, Theme, ThemeVariant};

fn dark_test_theme() -> Theme {
    Theme::builder("Dark Test")
        .variant(ThemeVariant::Dark)
        .token("bg.base", OpalineColor::new(30, 30, 46))
        .token("bg.panel", OpalineColor::new(36, 36, 54))
        .token("bg.highlight", OpalineColor::new(49, 50, 68))
        .token("bg.code", OpalineColor::new(24, 24, 37))
        .token("bg.selection", OpalineColor::new(69, 71, 90))
        .token("text.primary", OpalineColor::new(205, 214, 244))
        .token("text.secondary", OpalineColor::new(186, 194, 222))
        .token("text.muted", OpalineColor::new(147, 153, 178))
        .token("accent.primary", OpalineColor::new(203, 166, 247))
        .token("accent.secondary", OpalineColor::new(137, 180, 250))
        .token("border.focused", OpalineColor::new(203, 166, 247))
        .token("border.unfocused", OpalineColor::new(88, 91, 112))
        .token("success", OpalineColor::new(166, 227, 161))
        .token("warning", OpalineColor::new(249, 226, 175))
        .token("error", OpalineColor::new(243, 139, 168))
        .build()
}

#[test]
fn color_to_iced() {
    let c = OpalineColor::new(225, 53, 255);
    let ic: Color = c.into();
    assert_eq!(ic, Color::from_rgb8(225, 53, 255));
}

#[test]
fn color_ref_to_iced() {
    let c = OpalineColor::new(128, 255, 234);
    let ic: Color = (&c).into();
    assert_eq!(ic, Color::from_rgb8(128, 255, 234));
}

#[test]
fn palette_maps_required_slots() {
    let theme = dark_test_theme();
    let p = to_iced_palette(&theme);

    assert_eq!(p.background, Color::from_rgb8(30, 30, 46));
    assert_eq!(p.text, Color::from_rgb8(205, 214, 244));
    assert_eq!(p.primary, Color::from_rgb8(203, 166, 247));
    assert_eq!(p.success, Color::from_rgb8(166, 227, 161));
    assert_eq!(p.warning, Color::from_rgb8(249, 226, 175));
    assert_eq!(p.danger, Color::from_rgb8(243, 139, 168));
}

#[test]
fn extended_palette_marks_dark_variant() {
    let theme = dark_test_theme();
    let ext = to_iced_extended(&theme);
    assert!(ext.is_dark);
}

#[test]
fn extended_palette_marks_light_variant() {
    let theme = Theme::builder("Light Test")
        .variant(ThemeVariant::Light)
        .token("bg.base", OpalineColor::new(239, 241, 245))
        .token("text.primary", OpalineColor::new(76, 79, 105))
        .token("accent.primary", OpalineColor::new(136, 57, 239))
        .token("success", OpalineColor::new(64, 160, 43))
        .token("warning", OpalineColor::new(223, 142, 29))
        .token("error", OpalineColor::new(210, 15, 57))
        .build();

    let ext = to_iced_extended(&theme);
    assert!(!ext.is_dark);
}

#[test]
fn custom_theme_carries_full_palette() {
    let theme = dark_test_theme();
    let expected = to_iced_palette(&theme);
    let custom = to_iced_custom(&theme);

    // Wrap in iced_core's Theme so we can read the palette back through
    // the public surface that downstream apps actually use.
    let iced_theme = IcedTheme::Custom(Arc::new(custom));
    assert_eq!(iced_theme.palette(), expected);
    assert_eq!(iced_theme.name(), "Dark Test");
    assert!(iced_theme.extended_palette().is_dark);
}

#[cfg(feature = "builtin-themes")]
#[test]
fn every_builtin_converts_to_iced() {
    use opaline::builtins::{builtin_names, load_by_name};

    for &(id, _display) in builtin_names() {
        let theme = load_by_name(id).unwrap_or_else(|| panic!("builtin {id} should load"));
        let palette = to_iced_palette(&theme);
        let extended = to_iced_extended(&theme);
        let custom = to_iced_custom(&theme);

        assert_eq!(
            extended.is_dark,
            theme.is_dark(),
            "is_dark mismatch for {id}"
        );

        let iced_theme = IcedTheme::Custom(Arc::new(custom));
        assert_eq!(iced_theme.palette(), palette, "palette mismatch for {id}");
        assert_eq!(iced_theme.name(), theme.meta.name, "name mismatch for {id}");
    }
}
