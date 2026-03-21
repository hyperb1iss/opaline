#![cfg(feature = "crossterm")]

use crossterm::style::{Attribute, Color, ContentStyle};

use opaline::{OpalineColor, OpalineStyle, Theme};

#[test]
fn color_to_crossterm() {
    let c = OpalineColor::new(225, 53, 255);
    let ct: Color = c.into();
    assert_eq!(ct, Color::Rgb { r: 225, g: 53, b: 255 });
}

#[test]
fn color_ref_to_crossterm() {
    let c = OpalineColor::new(128, 255, 234);
    let ct: Color = (&c).into();
    assert_eq!(ct, Color::Rgb { r: 128, g: 255, b: 234 });
}

#[test]
fn style_fg_only() {
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255));
    let ct: ContentStyle = s.into();
    assert_eq!(ct.foreground_color, Some(Color::Rgb { r: 225, g: 53, b: 255 }));
    assert_eq!(ct.background_color, None);
}

#[test]
fn style_fg_and_bg() {
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255))
        .with_bg(OpalineColor::new(18, 18, 24));
    let ct: ContentStyle = s.into();
    assert_eq!(ct.foreground_color, Some(Color::Rgb { r: 225, g: 53, b: 255 }));
    assert_eq!(ct.background_color, Some(Color::Rgb { r: 18, g: 18, b: 24 }));
}

#[test]
fn style_all_nine_modifiers() {
    let s = OpalineStyle::new()
        .bold()
        .dim()
        .italic()
        .underline()
        .slow_blink()
        .rapid_blink()
        .reversed()
        .hidden()
        .crossed_out();

    let ct: ContentStyle = s.into();
    assert!(ct.attributes.has(Attribute::Bold));
    assert!(ct.attributes.has(Attribute::Dim));
    assert!(ct.attributes.has(Attribute::Italic));
    assert!(ct.attributes.has(Attribute::Underlined));
    assert!(ct.attributes.has(Attribute::SlowBlink));
    assert!(ct.attributes.has(Attribute::RapidBlink));
    assert!(ct.attributes.has(Attribute::Reverse));
    assert!(ct.attributes.has(Attribute::Hidden));
    assert!(ct.attributes.has(Attribute::CrossedOut));
}

#[test]
fn style_no_modifiers_empty() {
    let s = OpalineStyle::new();
    let ct: ContentStyle = s.into();
    assert!(ct.attributes.is_empty());
}

#[test]
fn style_ref_conversion() {
    let s = OpalineStyle::fg(OpalineColor::new(80, 250, 123)).bold();
    let ct: ContentStyle = (&s).into();
    assert_eq!(ct.foreground_color, Some(Color::Rgb { r: 80, g: 250, b: 123 }));
    assert!(ct.attributes.has(Attribute::Bold));
}

#[test]
fn theme_crossterm_styled() {
    let theme = Theme::builder("Test")
        .style("keyword", OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold())
        .build();

    let styled = theme.crossterm_styled("keyword", "fn");
    let output = format!("{styled}");
    assert!(output.contains("fn"));
}

#[cfg(feature = "gradients")]
mod gradient_tests {
    use opaline::{Gradient, OpalineColor};

    #[test]
    fn gradient_styled_basic() {
        let gradient = Gradient::new(vec![
            OpalineColor::new(255, 0, 0),
            OpalineColor::new(0, 0, 255),
        ]);

        let styled = opaline::adapters::crossterm::gradient_styled("AB", &gradient);
        assert_eq!(styled.len(), 2);
    }

    #[test]
    fn gradient_styled_empty() {
        let gradient = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
        let styled = opaline::adapters::crossterm::gradient_styled("", &gradient);
        assert!(styled.is_empty());
    }

    #[test]
    fn gradient_bar_basic() {
        let gradient = Gradient::new(vec![
            OpalineColor::new(255, 0, 0),
            OpalineColor::new(0, 255, 0),
        ]);

        let bar = opaline::adapters::crossterm::gradient_bar(5, '█', &gradient);
        assert_eq!(bar.len(), 5);
    }

    #[test]
    fn gradient_bar_zero_width() {
        let gradient = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
        let bar = opaline::adapters::crossterm::gradient_bar(0, '█', &gradient);
        assert!(bar.is_empty());
    }
}
