use opaline::{OpalineColor, OpalineStyle};
use pretty_assertions::assert_eq;

// ═══════════════════════════════════════════════════════════════════════════════
// Construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn new_is_empty() {
    let s = OpalineStyle::new();
    assert_eq!(s.fg, None);
    assert_eq!(s.bg, None);
    assert!(!s.bold);
    assert!(!s.dim);
    assert!(!s.italic);
    assert!(!s.underline);
    assert!(!s.slow_blink);
    assert!(!s.rapid_blink);
    assert!(!s.reversed);
    assert!(!s.hidden);
    assert!(!s.crossed_out);
}

#[test]
fn default_matches_new() {
    assert_eq!(OpalineStyle::default(), OpalineStyle::new());
}

#[test]
fn fg_constructor() {
    let c = OpalineColor::new(255, 0, 0);
    let s = OpalineStyle::fg(c);
    assert_eq!(s.fg, Some(c));
    assert_eq!(s.bg, None);
    assert!(!s.bold);
}

#[test]
fn bg_constructor() {
    let c = OpalineColor::new(0, 0, 255);
    let s = OpalineStyle::bg(c);
    assert_eq!(s.fg, None);
    assert_eq!(s.bg, Some(c));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Builder chain — all 9 modifiers
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn builder_chain_basic() {
    let fg = OpalineColor::new(225, 53, 255);
    let bg = OpalineColor::new(18, 18, 24);
    let s = OpalineStyle::new()
        .with_fg(fg)
        .with_bg(bg)
        .bold()
        .italic();

    assert_eq!(s.fg, Some(fg));
    assert_eq!(s.bg, Some(bg));
    assert!(s.bold);
    assert!(s.italic);
    assert!(!s.underline);
    assert!(!s.dim);
}

#[test]
fn builder_chain_all_nine_modifiers() {
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

    assert!(s.bold);
    assert!(s.dim);
    assert!(s.italic);
    assert!(s.underline);
    assert!(s.slow_blink);
    assert!(s.rapid_blink);
    assert!(s.reversed);
    assert!(s.hidden);
    assert!(s.crossed_out);
}

#[test]
fn builder_each_modifier_independent() {
    // Each modifier builder should only set its own flag
    let slow_blink = OpalineStyle::new().slow_blink();
    assert!(slow_blink.slow_blink);
    assert!(!slow_blink.rapid_blink);
    assert!(!slow_blink.bold);

    let rapid_blink = OpalineStyle::new().rapid_blink();
    assert!(rapid_blink.rapid_blink);
    assert!(!rapid_blink.slow_blink);

    let reversed = OpalineStyle::new().reversed();
    assert!(reversed.reversed);
    assert!(!reversed.hidden);

    let hidden = OpalineStyle::new().hidden();
    assert!(hidden.hidden);
    assert!(!hidden.reversed);

    let crossed = OpalineStyle::new().crossed_out();
    assert!(crossed.crossed_out);
    assert!(!crossed.underline);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Merge
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn merge_other_takes_precedence_for_colors() {
    let base = OpalineStyle::fg(OpalineColor::new(255, 0, 0))
        .with_bg(OpalineColor::new(0, 0, 0));
    let overlay = OpalineStyle::fg(OpalineColor::new(0, 255, 0));

    let merged = base.merge(&overlay);
    assert_eq!(merged.fg, Some(OpalineColor::new(0, 255, 0)));
    // bg comes from base since overlay doesn't set it
    assert_eq!(merged.bg, Some(OpalineColor::new(0, 0, 0)));
}

#[test]
fn merge_booleans_are_ored() {
    let base = OpalineStyle::new().bold();
    let overlay = OpalineStyle::new().italic();

    let merged = base.merge(&overlay);
    assert!(merged.bold);
    assert!(merged.italic);
}

#[test]
fn merge_all_nine_modifiers_ored() {
    let base = OpalineStyle::new()
        .bold()
        .italic()
        .underline()
        .slow_blink()
        .reversed();
    let overlay = OpalineStyle::new()
        .dim()
        .rapid_blink()
        .hidden()
        .crossed_out();

    let merged = base.merge(&overlay);
    assert!(merged.bold);
    assert!(merged.dim);
    assert!(merged.italic);
    assert!(merged.underline);
    assert!(merged.slow_blink);
    assert!(merged.rapid_blink);
    assert!(merged.reversed);
    assert!(merged.hidden);
    assert!(merged.crossed_out);
}

#[test]
fn merge_empty_preserves_base() {
    let base = OpalineStyle::fg(OpalineColor::new(255, 0, 0))
        .bold()
        .dim()
        .slow_blink()
        .reversed()
        .crossed_out();
    let merged = base.merge(&OpalineStyle::new());
    assert_eq!(merged, base);
}

#[test]
fn merge_onto_empty_is_overlay() {
    let overlay = OpalineStyle::fg(OpalineColor::new(0, 255, 0))
        .italic()
        .hidden();
    let merged = OpalineStyle::new().merge(&overlay);
    assert_eq!(merged, overlay);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Equality
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn eq_considers_all_fields() {
    let a = OpalineStyle::new().bold().slow_blink();
    let b = OpalineStyle::new().bold().slow_blink();
    let c = OpalineStyle::new().bold().rapid_blink();

    assert_eq!(a, b);
    assert_ne!(a, c);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Serde round-trip
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn serde_round_trip_all_modifiers() {
    let original = OpalineStyle::new()
        .with_fg(OpalineColor::new(255, 0, 0))
        .with_bg(OpalineColor::new(0, 0, 255))
        .bold()
        .dim()
        .italic()
        .underline()
        .slow_blink()
        .rapid_blink()
        .reversed()
        .hidden()
        .crossed_out();

    let json = serde_json::to_string(&original).expect("serialize");
    let deserialized: OpalineStyle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn serde_defaults_when_missing() {
    // Missing modifier fields should default to false
    let json = r#"{"fg":{"r":255,"g":0,"b":0}}"#;
    let s: OpalineStyle = serde_json::from_str(json).expect("deserialize");
    assert_eq!(s.fg, Some(OpalineColor::new(255, 0, 0)));
    assert!(!s.bold);
    assert!(!s.slow_blink);
    assert!(!s.reversed);
    assert!(!s.hidden);
    assert!(!s.crossed_out);
}
