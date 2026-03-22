#![cfg(feature = "css")]

use opaline::{OpalineColor, OpalineStyle, Theme};

#[test]
fn css_vars_contain_tokens() {
    let theme = Theme::builder("Test")
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .token("bg.base", OpalineColor::new(18, 18, 24))
        .build();

    let css = opaline::generate_css_vars(&theme);
    assert!(css.starts_with(":root {"));
    assert!(css.ends_with('}'));
    assert!(css.contains("--opaline-accent-primary: #e135ff;"));
    assert!(css.contains("--opaline-bg-base: #121218;"));
}

#[test]
fn css_classes_from_styles() {
    let theme = Theme::builder("Test")
        .style(
            "keyword",
            OpalineStyle::fg(OpalineColor::new(225, 53, 255))
                .bold()
                .italic(),
        )
        .style(
            "status_badge",
            OpalineStyle::fg(OpalineColor::new(80, 250, 123)),
        )
        .build();

    let css = opaline::generate_css_classes(&theme);
    assert!(css.contains(".opaline-keyword {"));
    assert!(css.contains("color: #e135ff;"));
    assert!(css.contains("font-weight: bold;"));
    assert!(css.contains("font-style: italic;"));
    assert!(css.contains(".opaline-status-badge {"));
}

#[test]
fn css_identifiers_escape_arbitrary_names() {
    let theme = Theme::builder("Test")
        .token("1accent.primary", OpalineColor::new(225, 53, 255))
        .style(
            "bad name",
            OpalineStyle::fg(OpalineColor::new(80, 250, 123)),
        )
        .style(
            "foo/bar",
            OpalineStyle::fg(OpalineColor::new(128, 255, 234)),
        )
        .build();

    let vars = opaline::generate_css_vars(&theme);
    assert!(vars.contains("--opaline-\\31 accent-primary: #e135ff;"));

    let classes = opaline::generate_css_classes(&theme);
    assert!(classes.contains(".opaline-bad\\20 name {"));
    assert!(classes.contains(".opaline-foo\\2f bar {"));
}

#[test]
fn css_stylesheet_combines_vars_and_classes() {
    let theme = Theme::builder("Test")
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .style("keyword", OpalineStyle::fg(OpalineColor::new(225, 53, 255)))
        .build();

    let css = opaline::generate_stylesheet(&theme);
    assert!(css.contains(":root {"));
    assert!(css.contains(".opaline-keyword {"));
}

#[cfg(feature = "gradients")]
#[test]
fn css_vars_include_gradients() {
    use opaline::Gradient;

    let theme = Theme::builder("Test")
        .gradient(
            "primary",
            Gradient::new(vec![
                OpalineColor::new(225, 53, 255),
                OpalineColor::new(128, 255, 234),
            ]),
        )
        .build();

    let css = opaline::generate_css_vars(&theme);
    assert!(
        css.contains("--opaline-gradient-primary: linear-gradient(to right, #e135ff, #80ffea);")
    );
}

#[test]
fn css_empty_theme_produces_empty_root() {
    let theme = Theme::builder("Empty").build();
    let css = opaline::generate_css_vars(&theme);
    assert_eq!(css, ":root {\n}");
}

#[test]
fn css_decorations_combined() {
    let theme = Theme::builder("Test")
        .style("strikelink", OpalineStyle::new().underline().crossed_out())
        .build();

    let css = opaline::generate_css_classes(&theme);
    assert!(css.contains("text-decoration: underline line-through;"));
}
