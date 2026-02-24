use opaline::{Gradient, OpalineColor};
use pretty_assertions::assert_eq;

#[test]
fn single_stop_always_returns_that_color() {
    let g = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    assert_eq!(g.at(0.0), OpalineColor::new(255, 0, 0));
    assert_eq!(g.at(0.5), OpalineColor::new(255, 0, 0));
    assert_eq!(g.at(1.0), OpalineColor::new(255, 0, 0));
}

#[test]
fn two_stop_endpoints() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(255, 255, 255);
    let g = Gradient::new(vec![a, b]);

    assert_eq!(g.at(0.0), a);
    assert_eq!(g.at(1.0), b);
}

#[test]
fn two_stop_midpoint() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(254, 254, 254);
    let g = Gradient::new(vec![a, b]);

    let mid = g.at(0.5);
    assert_eq!(mid, OpalineColor::new(127, 127, 127));
}

#[test]
fn multi_stop_segments() {
    let r = OpalineColor::new(255, 0, 0);
    let g_color = OpalineColor::new(0, 255, 0);
    let b = OpalineColor::new(0, 0, 255);
    let grad = Gradient::new(vec![r, g_color, b]);

    // t=0.0 → first stop
    assert_eq!(grad.at(0.0), r);
    // t=0.5 → second stop (middle)
    assert_eq!(grad.at(0.5), g_color);
    // t=1.0 → third stop
    assert_eq!(grad.at(1.0), b);
}

#[test]
fn at_clamps_below_zero() {
    let g = Gradient::new(vec![OpalineColor::new(100, 100, 100)]);
    assert_eq!(g.at(-1.0), OpalineColor::new(100, 100, 100));
}

#[test]
fn at_clamps_above_one() {
    let g = Gradient::new(vec![OpalineColor::new(200, 200, 200)]);
    assert_eq!(g.at(2.0), OpalineColor::new(200, 200, 200));
}

#[test]
fn generate_zero_returns_empty() {
    let g = Gradient::new(vec![OpalineColor::new(255, 0, 0)]);
    assert!(g.generate(0).is_empty());
}

#[test]
fn generate_one_returns_midpoint() {
    let g = Gradient::new(vec![
        OpalineColor::new(0, 0, 0),
        OpalineColor::new(254, 254, 254),
    ]);
    let result = g.generate(1);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], OpalineColor::new(127, 127, 127));
}

#[test]
fn generate_n_correct_count() {
    let g = Gradient::new(vec![
        OpalineColor::new(0, 0, 0),
        OpalineColor::new(255, 255, 255),
    ]);
    assert_eq!(g.generate(5).len(), 5);
    assert_eq!(g.generate(10).len(), 10);
}

#[test]
fn generate_endpoints_match() {
    let a = OpalineColor::new(0, 0, 0);
    let b = OpalineColor::new(255, 255, 255);
    let g = Gradient::new(vec![a, b]);
    let result = g.generate(3);

    assert_eq!(result[0], a);
    assert_eq!(result[2], b);
}

#[test]
fn len_and_is_empty() {
    let g = Gradient::new(vec![OpalineColor::FALLBACK]);
    assert_eq!(g.len(), 1);
    assert!(!g.is_empty());
}

#[test]
fn stops_accessor() {
    let colors = vec![OpalineColor::new(255, 0, 0), OpalineColor::new(0, 255, 0)];
    let g = Gradient::new(colors.clone());
    assert_eq!(g.stops(), &colors);
}

#[test]
fn default_is_single_fallback() {
    let g = Gradient::default();
    assert_eq!(g.len(), 1);
    assert_eq!(g.at(0.0), OpalineColor::FALLBACK);
}

#[test]
#[should_panic(expected = "gradient must have at least one stop")]
fn empty_stops_panics() {
    Gradient::new(vec![]);
}
