use opaline::{ColorParseError, OpalineColor};
use pretty_assertions::assert_eq;

#[test]
fn new_creates_color() {
    let c = OpalineColor::new(255, 128, 0);
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 0);
}

#[test]
fn from_hex_valid() {
    let c = OpalineColor::from_hex("#e135ff").expect("valid hex");
    assert_eq!(c, OpalineColor::new(225, 53, 255));
}

#[test]
fn from_hex_lowercase() {
    let c = OpalineColor::from_hex("#80ffea").expect("valid hex");
    assert_eq!(c, OpalineColor::new(128, 255, 234));
}

#[test]
fn from_hex_black() {
    let c = OpalineColor::from_hex("#000000").expect("valid hex");
    assert_eq!(c, OpalineColor::new(0, 0, 0));
}

#[test]
fn from_hex_white() {
    let c = OpalineColor::from_hex("#ffffff").expect("valid hex");
    assert_eq!(c, OpalineColor::new(255, 255, 255));
}

#[test]
fn from_hex_trims_whitespace() {
    let c = OpalineColor::from_hex("  #ff0000  ").expect("valid hex with whitespace");
    assert_eq!(c, OpalineColor::new(255, 0, 0));
}

#[test]
fn from_hex_rejects_short() {
    let err = OpalineColor::from_hex("#fff").expect_err("should fail");
    assert!(matches!(err, ColorParseError::InvalidLength(4)));
}

#[test]
fn from_hex_rejects_no_hash() {
    let err = OpalineColor::from_hex("ff0000").expect_err("should fail");
    assert!(matches!(err, ColorParseError::InvalidLength(_)));
}

#[test]
fn from_hex_rejects_invalid_chars() {
    let err = OpalineColor::from_hex("#gggggg").expect_err("should fail");
    assert!(matches!(err, ColorParseError::InvalidHex(_)));
}

#[test]
fn to_hex_roundtrip() {
    let c = OpalineColor::new(225, 53, 255);
    assert_eq!(c.to_hex(), "#e135ff");
    assert_eq!(OpalineColor::from_hex(&c.to_hex()).expect("roundtrip"), c);
}

#[test]
fn to_rgb_tuple() {
    let c = OpalineColor::new(10, 20, 30);
    assert_eq!(c.to_rgb_tuple(), (10, 20, 30));
}

#[test]
fn display_format() {
    let c = OpalineColor::new(128, 255, 234);
    assert_eq!(format!("{c}"), "#80ffea");
}

#[test]
fn from_str_parses() {
    let c: OpalineColor = "#50fa7b".parse().expect("valid hex via FromStr");
    assert_eq!(c, OpalineColor::new(80, 250, 123));
}

#[test]
fn default_is_fallback() {
    assert_eq!(OpalineColor::default(), OpalineColor::FALLBACK);
    assert_eq!(OpalineColor::default(), OpalineColor::new(128, 128, 128));
}

#[test]
fn lerp_at_zero() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(255, 255, 255);
    assert_eq!(a.lerp(b, 0.0), a);
}

#[test]
fn lerp_at_one() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(255, 255, 255);
    assert_eq!(a.lerp(b, 1.0), b);
}

#[test]
fn lerp_midpoint() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(254, 254, 254);
    let mid = a.lerp(b, 0.5);
    assert_eq!(mid, OpalineColor::new(127, 127, 127));
}

#[test]
fn lerp_clamps_below_zero() {
    let a = OpalineColor::new(100, 100, 100);
    let b = OpalineColor::new(200, 200, 200);
    assert_eq!(a.lerp(b, -1.0), a);
}

#[test]
fn lerp_clamps_above_one() {
    let a = OpalineColor::new(100, 100, 100);
    let b = OpalineColor::new(200, 200, 200);
    assert_eq!(a.lerp(b, 2.0), b);
}
