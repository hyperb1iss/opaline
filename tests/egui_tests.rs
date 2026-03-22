#![cfg(feature = "egui")]

use egui::Color32;

use opaline::{OpalineColor, Theme, ThemeVariant};

#[test]
fn color_to_egui() {
    let c = OpalineColor::new(225, 53, 255);
    let ec: Color32 = c.into();
    assert_eq!(ec, Color32::from_rgb(225, 53, 255));
}

#[test]
fn color_ref_to_egui() {
    let c = OpalineColor::new(128, 255, 234);
    let ec: Color32 = (&c).into();
    assert_eq!(ec, Color32::from_rgb(128, 255, 234));
}

#[test]
fn dark_theme_produces_dark_visuals() {
    let theme = Theme::builder("Dark Test")
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
        .token("warning", OpalineColor::new(249, 226, 175))
        .token("error", OpalineColor::new(243, 139, 168))
        .build();

    let v = opaline::adapters::egui::to_egui_visuals(&theme);
    assert!(v.dark_mode);
    assert_eq!(v.panel_fill, Color32::from_rgb(30, 30, 46));
    assert_eq!(v.window_fill, Color32::from_rgb(36, 36, 54));
    assert_eq!(v.hyperlink_color, Color32::from_rgb(203, 166, 247));
    assert_eq!(v.error_fg_color, Color32::from_rgb(243, 139, 168));
    assert_eq!(v.warn_fg_color, Color32::from_rgb(249, 226, 175));
}

#[test]
fn light_theme_produces_light_visuals() {
    let theme = Theme::builder("Light Test")
        .variant(ThemeVariant::Light)
        .token("bg.base", OpalineColor::new(239, 241, 245))
        .token("bg.panel", OpalineColor::new(230, 233, 239))
        .token("bg.highlight", OpalineColor::new(204, 208, 218))
        .token("bg.code", OpalineColor::new(220, 224, 232))
        .token("bg.selection", OpalineColor::new(172, 176, 190))
        .token("text.primary", OpalineColor::new(76, 79, 105))
        .token("text.secondary", OpalineColor::new(92, 95, 119))
        .token("text.muted", OpalineColor::new(108, 111, 133))
        .token("accent.primary", OpalineColor::new(136, 57, 239))
        .token("accent.secondary", OpalineColor::new(30, 102, 245))
        .token("border.focused", OpalineColor::new(136, 57, 239))
        .token("border.unfocused", OpalineColor::new(156, 160, 176))
        .token("warning", OpalineColor::new(223, 142, 29))
        .token("error", OpalineColor::new(210, 15, 57))
        .build();

    let v = opaline::adapters::egui::to_egui_visuals(&theme);
    assert!(!v.dark_mode);
    assert_eq!(v.panel_fill, Color32::from_rgb(239, 241, 245));
}

#[test]
fn widget_visuals_use_theme_colors() {
    let theme = Theme::builder("Test")
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
        .token("warning", OpalineColor::new(249, 226, 175))
        .token("error", OpalineColor::new(243, 139, 168))
        .build();

    let v = opaline::adapters::egui::to_egui_visuals(&theme);

    // Noninteractive uses base bg
    assert_eq!(v.widgets.noninteractive.bg_fill, Color32::from_rgb(30, 30, 46));
    // Inactive uses panel bg
    assert_eq!(v.widgets.inactive.bg_fill, Color32::from_rgb(36, 36, 54));
    // Hovered uses highlight bg
    assert_eq!(v.widgets.hovered.bg_fill, Color32::from_rgb(49, 50, 68));
    // Active uses selection bg
    assert_eq!(v.widgets.active.bg_fill, Color32::from_rgb(69, 71, 90));
}

#[test]
fn selection_uses_theme_colors() {
    let theme = Theme::builder("Test")
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
        .token("warning", OpalineColor::new(249, 226, 175))
        .token("error", OpalineColor::new(243, 139, 168))
        .build();

    let v = opaline::adapters::egui::to_egui_visuals(&theme);
    assert_eq!(v.selection.bg_fill, Color32::from_rgb(69, 71, 90));
    assert_eq!(v.selection.stroke.color, Color32::from_rgb(203, 166, 247));
}

#[cfg(feature = "builtin-themes")]
#[test]
fn builtin_theme_converts_to_egui() {
    let theme = Theme::default();
    let v = opaline::adapters::egui::to_egui_visuals(&theme);
    assert!(v.dark_mode);
    // Should have non-default colors
    assert_ne!(v.panel_fill, Color32::from_rgb(0, 0, 0));
}
