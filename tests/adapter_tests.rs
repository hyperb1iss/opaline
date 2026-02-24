use opaline::{Gradient, OpalineColor, OpalineStyle, ThemeRatatuiExt};
use ratatui_core::style::{Color, Modifier, Style};

#[test]
fn color_to_ratatui() {
    let c = OpalineColor::new(225, 53, 255);
    let rc: Color = c.into();
    assert_eq!(rc, Color::Rgb(225, 53, 255));
}

#[test]
fn color_ref_to_ratatui() {
    let c = OpalineColor::new(128, 255, 234);
    let rc: Color = (&c).into();
    assert_eq!(rc, Color::Rgb(128, 255, 234));
}

#[test]
fn style_to_ratatui_fg_only() {
    let s = OpalineStyle::fg(OpalineColor::new(255, 0, 0));
    let rs: Style = s.into();
    assert_eq!(rs.fg, Some(Color::Rgb(255, 0, 0)));
    assert_eq!(rs.bg, None);
}

#[test]
fn style_to_ratatui_with_modifiers() {
    let s = OpalineStyle::new()
        .with_fg(OpalineColor::new(225, 53, 255))
        .bold()
        .italic();
    let rs: Style = s.into();

    assert_eq!(rs.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(rs.add_modifier.contains(Modifier::BOLD));
    assert!(rs.add_modifier.contains(Modifier::ITALIC));
    assert!(!rs.add_modifier.contains(Modifier::UNDERLINED));
}

#[test]
fn style_to_ratatui_all_modifiers() {
    let s = OpalineStyle::new().bold().italic().underline().dim();
    let rs: Style = s.into();

    assert!(rs.add_modifier.contains(Modifier::BOLD));
    assert!(rs.add_modifier.contains(Modifier::ITALIC));
    assert!(rs.add_modifier.contains(Modifier::UNDERLINED));
    assert!(rs.add_modifier.contains(Modifier::DIM));
}

#[test]
fn style_ref_to_ratatui() {
    let s = OpalineStyle::fg(OpalineColor::new(0, 255, 0)).bold();
    let rs: Style = (&s).into();
    assert_eq!(rs.fg, Some(Color::Rgb(0, 255, 0)));
    assert!(rs.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn theme_ratatui_ext_color() {
    let theme = opaline::Theme::default();
    let color = theme.ratatui_color("accent.primary");
    assert_eq!(color, Color::Rgb(225, 53, 255));
}

#[test]
fn theme_ratatui_ext_style() {
    let theme = opaline::Theme::default();
    let style = theme.ratatui_style("keyword");
    assert_eq!(style.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn theme_ratatui_ext_gradient() {
    let theme = opaline::Theme::default();
    let color = theme.ratatui_gradient("primary", 0.0);
    assert_eq!(color, Color::Rgb(225, 53, 255));
}

#[test]
fn gradient_spans_basic() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 0, 255),
    ]);
    let spans = opaline::adapters::ratatui::gradient_spans("Hi", &grad);
    assert_eq!(spans.len(), 2);
}

#[test]
fn gradient_spans_empty_text() {
    let grad = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    let spans = opaline::adapters::ratatui::gradient_spans("", &grad);
    assert!(spans.is_empty());
}

#[test]
fn gradient_line_basic() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 255, 0),
    ]);
    let spans = opaline::adapters::ratatui::gradient_line(5, '─', &grad);
    assert_eq!(spans.len(), 5);
}

#[test]
fn gradient_line_zero_width() {
    let grad = Gradient::new(vec![OpalineColor::FALLBACK]);
    let spans = opaline::adapters::ratatui::gradient_line(0, '─', &grad);
    assert!(spans.is_empty());
}
