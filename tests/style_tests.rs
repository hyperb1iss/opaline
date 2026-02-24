use opaline::{OpalineColor, OpalineStyle};
use pretty_assertions::assert_eq;

#[test]
fn new_is_empty() {
    let s = OpalineStyle::new();
    assert_eq!(s.fg, None);
    assert_eq!(s.bg, None);
    assert!(!s.bold);
    assert!(!s.italic);
    assert!(!s.underline);
    assert!(!s.dim);
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
}

#[test]
fn bg_constructor() {
    let c = OpalineColor::new(0, 0, 255);
    let s = OpalineStyle::bg(c);
    assert_eq!(s.fg, None);
    assert_eq!(s.bg, Some(c));
}

#[test]
fn builder_chain() {
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
fn merge_empty_preserves_base() {
    let base = OpalineStyle::fg(OpalineColor::new(255, 0, 0)).bold().dim();
    let merged = base.merge(&OpalineStyle::new());
    assert_eq!(merged, base);
}
