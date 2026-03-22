use opaline::{OpalineColor, Theme};

#[test]
fn register_default_token_does_not_shadow_palette_name() {
    let original = OpalineColor::new(1, 2, 3);
    let mut theme = Theme::builder("Test")
        .palette("accent.primary", original)
        .build();

    theme.register_default_token("accent.primary", OpalineColor::new(9, 9, 9));

    assert_eq!(theme.color("accent.primary"), original);
    assert!(!theme.token_names().contains(&"accent.primary"));
}

#[test]
fn register_default_token_inserts_when_missing() {
    let color = OpalineColor::new(1, 2, 3);
    let mut theme = Theme::builder("Test").build();

    theme.register_default_token("accent.primary", color);

    assert_eq!(theme.try_color("accent.primary"), Some(color));
}
