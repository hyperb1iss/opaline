#![cfg(feature = "owo-colors")]

use owo_colors::OwoColorize as _;

use opaline::{OpalineColor, OpalineStyle, OwoThemeExt as _, Theme};

#[test]
fn style_to_owo() {
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold();
    let owo: owo_colors::Style = s.into();
    let output = format!("{}", "test".style(owo));
    // Should contain ANSI escape codes
    assert!(output.contains("test"));
    assert!(output.len() > 4);
}

#[test]
fn style_ref_to_owo() {
    let s = OpalineStyle::fg(OpalineColor::new(128, 255, 234)).italic();
    let owo: owo_colors::Style = (&s).into();
    let output = format!("{}", "hello".style(owo));
    assert!(output.contains("hello"));
}

#[test]
fn empty_style_produces_unstyled() {
    let s = OpalineStyle::new();
    let owo: owo_colors::Style = s.into();
    let output = format!("{}", "plain".style(owo));
    assert_eq!(output, "plain");
}

#[test]
fn all_modifiers_applied() {
    let s = OpalineStyle::new()
        .bold()
        .dim()
        .italic()
        .underline()
        .slow_blink()
        .reversed()
        .hidden()
        .crossed_out();
    let owo: owo_colors::Style = s.into();
    let output = format!("{}", "x".style(owo));
    // At minimum should contain escape sequences
    assert!(output.len() > 1);
}

#[test]
fn theme_owo_style() {
    let theme = Theme::builder("Test")
        .style("keyword", OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold())
        .build();

    let style = theme.owo_style("keyword");
    let output = format!("{}", "fn".style(style));
    assert!(output.contains("fn"));
    assert!(output.len() > 2);
}

#[test]
fn theme_owo_fg() {
    let theme = Theme::builder("Test")
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .build();

    let style = theme.owo_fg("accent.primary");
    let output = format!("{}", "accent".style(style));
    assert!(output.contains("accent"));
}

#[test]
fn theme_owo_bg() {
    let theme = Theme::builder("Test")
        .token("bg.base", OpalineColor::new(18, 18, 24))
        .build();

    let style = theme.owo_bg("bg.base");
    let output = format!("{}", "bg".style(style));
    assert!(output.contains("bg"));
}

#[cfg(feature = "gradients")]
#[test]
fn gradient_string_basic() {
    use opaline::Gradient;

    let gradient = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 0, 255),
    ]);

    let output = opaline::adapters::owo_colors::gradient_string("hello", &gradient);
    assert!(output.contains('h'));
    assert!(output.contains('o'));
    // Should have ANSI codes
    assert!(output.len() > 5);
}

#[cfg(feature = "gradients")]
#[test]
fn gradient_string_empty() {
    use opaline::Gradient;

    let gradient = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    let output = opaline::adapters::owo_colors::gradient_string("", &gradient);
    assert!(output.is_empty());
}
