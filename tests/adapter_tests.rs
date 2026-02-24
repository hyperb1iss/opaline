use opaline::{Gradient, OpalineColor, OpalineStyle};
use ratatui_core::style::{Color, Modifier, Style, Styled};
use ratatui_core::text::{Line, Span, Text};

// ═══════════════════════════════════════════════════════════════════════════════
// Color → ratatui conversions
// ═══════════════════════════════════════════════════════════════════════════════

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
fn color_to_ratatui_style() {
    let c = OpalineColor::new(80, 250, 123);
    let style: Style = c.into();
    assert_eq!(style.fg, Some(Color::Rgb(80, 250, 123)));
    assert_eq!(style.bg, None);
}

#[test]
fn color_into_style_is_fg() {
    // OpalineColor as Into<Style> should set fg, not bg
    let c = OpalineColor::new(255, 106, 193);
    let style: Style = c.into();
    assert_eq!(style.fg, Some(Color::Rgb(255, 106, 193)));
    assert_eq!(style.bg, None);
    assert!(style.add_modifier.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// OpalineStyle → ratatui Style conversion
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn style_to_ratatui_fg_only() {
    let s = OpalineStyle::fg(OpalineColor::new(255, 0, 0));
    let rs: Style = s.into();
    assert_eq!(rs.fg, Some(Color::Rgb(255, 0, 0)));
    assert_eq!(rs.bg, None);
}

#[test]
fn style_to_ratatui_fg_and_bg() {
    let s = OpalineStyle::new()
        .with_fg(OpalineColor::new(255, 255, 255))
        .with_bg(OpalineColor::new(30, 30, 40));
    let rs: Style = s.into();
    assert_eq!(rs.fg, Some(Color::Rgb(255, 255, 255)));
    assert_eq!(rs.bg, Some(Color::Rgb(30, 30, 40)));
}

#[test]
fn style_to_ratatui_with_basic_modifiers() {
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
fn style_to_ratatui_all_nine_modifiers() {
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
    let rs: Style = s.into();

    assert!(rs.add_modifier.contains(Modifier::BOLD));
    assert!(rs.add_modifier.contains(Modifier::DIM));
    assert!(rs.add_modifier.contains(Modifier::ITALIC));
    assert!(rs.add_modifier.contains(Modifier::UNDERLINED));
    assert!(rs.add_modifier.contains(Modifier::SLOW_BLINK));
    assert!(rs.add_modifier.contains(Modifier::RAPID_BLINK));
    assert!(rs.add_modifier.contains(Modifier::REVERSED));
    assert!(rs.add_modifier.contains(Modifier::HIDDEN));
    assert!(rs.add_modifier.contains(Modifier::CROSSED_OUT));
}

#[test]
fn style_to_ratatui_no_modifiers_empty_bitfield() {
    let s = OpalineStyle::new();
    let rs: Style = s.into();
    assert!(rs.add_modifier.is_empty());
}

#[test]
fn style_to_ratatui_single_modifier_each() {
    // Test each modifier individually to catch bit-mapping errors
    let tests: Vec<(OpalineStyle, Modifier)> = vec![
        (OpalineStyle::new().bold(), Modifier::BOLD),
        (OpalineStyle::new().dim(), Modifier::DIM),
        (OpalineStyle::new().italic(), Modifier::ITALIC),
        (OpalineStyle::new().underline(), Modifier::UNDERLINED),
        (OpalineStyle::new().slow_blink(), Modifier::SLOW_BLINK),
        (OpalineStyle::new().rapid_blink(), Modifier::RAPID_BLINK),
        (OpalineStyle::new().reversed(), Modifier::REVERSED),
        (OpalineStyle::new().hidden(), Modifier::HIDDEN),
        (OpalineStyle::new().crossed_out(), Modifier::CROSSED_OUT),
    ];

    for (opaline_style, expected_modifier) in tests {
        let rs: Style = opaline_style.into();
        assert!(
            rs.add_modifier.contains(expected_modifier),
            "modifier {expected_modifier:?} should be set"
        );
        // Should only have that one modifier
        assert_eq!(
            rs.add_modifier, expected_modifier,
            "only {expected_modifier:?} should be set"
        );
    }
}

#[test]
fn style_ref_to_ratatui() {
    let s = OpalineStyle::fg(OpalineColor::new(0, 255, 0)).bold();
    let rs: Style = (&s).into();
    assert_eq!(rs.fg, Some(Color::Rgb(0, 255, 0)));
    assert!(rs.add_modifier.contains(Modifier::BOLD));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Styled trait — unlocks the Stylize fluent API
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn opaline_style_implements_styled() {
    // The Styled trait provides .style() and .set_style()
    let s = OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold();
    let ratatui_style: Style = s.style();
    assert_eq!(ratatui_style.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(ratatui_style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn styled_set_style_patches() {
    // set_style should merge the opaline style with the provided style
    let base = OpalineStyle::fg(OpalineColor::new(255, 0, 0)).bold();
    let patch = Style::default().bg(Color::Rgb(0, 0, 255));
    let result: Style = base.set_style(patch);

    assert_eq!(result.fg, Some(Color::Rgb(255, 0, 0)));
    assert_eq!(result.bg, Some(Color::Rgb(0, 0, 255)));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme inherent methods — zero-import ratatui bridge
// ═══════════════════════════════════════════════════════════════════════════════

fn test_theme() -> opaline::Theme {
    opaline::Theme::builder("Test Theme")
        .token("accent", OpalineColor::new(225, 53, 255))
        .token("bg.base", OpalineColor::new(18, 18, 24))
        .style(
            "keyword",
            OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold(),
        )
        .style(
            "comment",
            OpalineStyle::fg(OpalineColor::new(100, 100, 120)).italic(),
        )
        .gradient(
            "sunset",
            Gradient::new(vec![
                OpalineColor::new(255, 0, 0),
                OpalineColor::new(255, 165, 0),
                OpalineColor::new(255, 255, 0),
            ]),
        )
        .build()
}

#[test]
fn theme_color_into_ratatui() {
    let theme = test_theme();
    let color: Color = theme.color("accent").into();
    assert_eq!(color, Color::Rgb(225, 53, 255));
}

#[test]
fn theme_color_fallback_into_ratatui() {
    let theme = test_theme();
    let color: Color = theme.color("nonexistent").into();
    assert_eq!(color, Color::Rgb(128, 128, 128)); // FALLBACK
}

#[test]
fn theme_style_into_ratatui() {
    let theme = test_theme();
    let style: Style = theme.style("keyword").into();
    assert_eq!(style.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn theme_style_missing_returns_default() {
    let theme = test_theme();
    let style: Style = theme.style("nonexistent").into();
    assert_eq!(style, Style::default());
}

#[test]
fn theme_span() {
    let theme = test_theme();
    let span: Span = theme.span("keyword", "fn");
    assert_eq!(span.content, "fn");
    assert_eq!(span.style.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(span.style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn theme_span_owned_string() {
    let theme = test_theme();
    let content = String::from("hello");
    let span: Span = theme.span("comment", content);
    assert_eq!(span.content, "hello");
    assert!(span.style.add_modifier.contains(Modifier::ITALIC));
}

#[test]
fn theme_line() {
    let theme = test_theme();
    let line: Line = theme.line("keyword", "let x = 42;");
    // Line wraps a single span when styled
    assert_eq!(line.style.fg, Some(Color::Rgb(225, 53, 255)));
}

#[test]
fn theme_text() {
    let theme = test_theme();
    let text: Text = theme.text("comment", "// TODO");
    assert_eq!(text.style.fg, Some(Color::Rgb(100, 100, 120)));
    assert!(text.style.add_modifier.contains(Modifier::ITALIC));
}

#[test]
fn theme_gradient_into_ratatui() {
    let theme = test_theme();
    let color: Color = theme.gradient("sunset", 0.0).into();
    assert_eq!(color, Color::Rgb(255, 0, 0));
    let color: Color = theme.gradient("sunset", 1.0).into();
    assert_eq!(color, Color::Rgb(255, 255, 0));
}

#[test]
fn theme_gradient_missing_into_ratatui() {
    let theme = test_theme();
    let color: Color = theme.gradient("nonexistent", 0.5).into();
    assert_eq!(color, Color::Rgb(128, 128, 128)); // FALLBACK
}

#[test]
fn theme_gradient_text_basic() {
    let theme = test_theme();
    let line: Line = theme.gradient_text("sunset", "rainbow");
    // Each char gets its own span with gradient coloring
    assert_eq!(line.spans.len(), 7); // "rainbow" = 7 chars
}

#[test]
fn theme_gradient_text_missing_gradient() {
    let theme = test_theme();
    let line: Line = theme.gradient_text("nonexistent", "fallback text");
    // Missing gradient → raw line
    assert_eq!(line.spans.len(), 1);
}

#[test]
fn theme_gradient_text_empty_string() {
    let theme = test_theme();
    let line: Line = theme.gradient_text("sunset", "");
    assert!(line.spans.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gradient helper functions
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn gradient_spans_basic() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 0, 255),
    ]);
    let spans = opaline::adapters::ratatui::gradient_spans("Hi", &grad);
    assert_eq!(spans.len(), 2);

    // First char → red, last → blue
    assert_eq!(spans[0].style.fg, Some(Color::Rgb(255, 0, 0)));
    assert_eq!(spans[1].style.fg, Some(Color::Rgb(0, 0, 255)));
}

#[test]
fn gradient_spans_single_char() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 0, 255),
    ]);
    let spans = opaline::adapters::ratatui::gradient_spans("X", &grad);
    assert_eq!(spans.len(), 1);
    // Single char → midpoint
    assert_eq!(spans[0].content, "X");
}

#[test]
fn gradient_spans_empty_text() {
    let grad = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    let spans = opaline::adapters::ratatui::gradient_spans("", &grad);
    assert!(spans.is_empty());
}

#[test]
fn gradient_spans_unicode() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 255, 0),
    ]);
    let spans = opaline::adapters::ratatui::gradient_spans("AB", &grad);
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].content, "A");
    assert_eq!(spans[1].content, "B");
}

#[test]
fn gradient_line_basic() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 255, 0),
    ]);
    let spans = opaline::adapters::ratatui::gradient_line(5, '\u{2500}', &grad);
    assert_eq!(spans.len(), 5);
    for span in &spans {
        assert_eq!(span.content, "\u{2500}");
    }
}

#[test]
fn gradient_line_zero_width() {
    let grad = Gradient::new(vec![OpalineColor::FALLBACK]);
    let spans = opaline::adapters::ratatui::gradient_line(0, '\u{2500}', &grad);
    assert!(spans.is_empty());
}

#[test]
fn gradient_text_line_returns_line() {
    let grad = Gradient::new(vec![
        OpalineColor::new(225, 53, 255),
        OpalineColor::new(128, 255, 234),
    ]);
    let line: Line = opaline::adapters::ratatui::gradient_text_line("Opaline", &grad);
    assert_eq!(line.spans.len(), 7);
}

#[test]
fn gradient_bar_returns_line() {
    let grad = Gradient::new(vec![
        OpalineColor::new(255, 0, 0),
        OpalineColor::new(0, 0, 255),
    ]);
    let line: Line = opaline::adapters::ratatui::gradient_bar(10, '\u{2588}', &grad);
    assert_eq!(line.spans.len(), 10);
    for span in &line.spans {
        assert_eq!(span.content, "\u{2588}");
    }
}

#[test]
fn gradient_bar_zero_width_empty() {
    let grad = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    let line: Line = opaline::adapters::ratatui::gradient_bar(0, '\u{2588}', &grad);
    assert!(line.spans.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Integration: Builder → Theme → Ratatui
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn builder_to_ratatui_round_trip() {
    let theme = opaline::Theme::builder("Round Trip")
        .token("primary", OpalineColor::new(225, 53, 255))
        .style(
            "heading",
            OpalineStyle::fg(OpalineColor::new(225, 53, 255))
                .bold()
                .underline(),
        )
        .build();

    let color: Color = theme.color("primary").into();
    assert_eq!(color, Color::Rgb(225, 53, 255));

    let style: Style = theme.style("heading").into();
    assert_eq!(style.fg, Some(Color::Rgb(225, 53, 255)));
    assert!(style.add_modifier.contains(Modifier::BOLD));
    assert!(style.add_modifier.contains(Modifier::UNDERLINED));
}

#[test]
fn default_theme_ratatui_integration() {
    let theme = opaline::Theme::default();

    // Default theme should produce valid ratatui types
    let _color: Color = theme.color("accent.primary").into();
    let _style: Style = theme.style("keyword").into();
    let _span: Span = theme.span("keyword", "test");
    let _line: Line = theme.line("keyword", "test");
    let _text: Text = theme.text("keyword", "test");
}

// ═══════════════════════════════════════════════════════════════════════════════
// Modifier bits correctness (internal, but tested via public API)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn modifier_bits_match_ratatui() {
    // Verify our bit positions match ratatui's Modifier constants
    assert_eq!(Modifier::BOLD.bits(), 0b0000_0000_0001);
    assert_eq!(Modifier::DIM.bits(), 0b0000_0000_0010);
    assert_eq!(Modifier::ITALIC.bits(), 0b0000_0000_0100);
    assert_eq!(Modifier::UNDERLINED.bits(), 0b0000_0000_1000);
    assert_eq!(Modifier::SLOW_BLINK.bits(), 0b0000_0001_0000);
    assert_eq!(Modifier::RAPID_BLINK.bits(), 0b0000_0010_0000);
    assert_eq!(Modifier::REVERSED.bits(), 0b0000_0100_0000);
    assert_eq!(Modifier::HIDDEN.bits(), 0b0000_1000_0000);
    assert_eq!(Modifier::CROSSED_OUT.bits(), 0b0001_0000_0000);
}
