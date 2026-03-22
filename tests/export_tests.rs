use opaline::{OpalineColor, OpalineStyle, Theme, ThemeVariant, load_from_str};

#[test]
fn to_toml_round_trips() {
    let original = Theme::builder("Round Trip")
        .author("test")
        .variant(ThemeVariant::Dark)
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .token("bg.base", OpalineColor::new(18, 18, 24))
        .token("text.primary", OpalineColor::new(205, 214, 244))
        .style(
            "keyword",
            OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold(),
        )
        .build();

    let toml_str = original.to_toml().expect("serialization succeeds");
    let reloaded = load_from_str(&toml_str, None).expect("deserialization succeeds");

    assert_eq!(reloaded.meta.name, "Round Trip");
    assert_eq!(reloaded.meta.author.as_deref(), Some("test"));
    assert!(reloaded.is_dark());
    assert_eq!(
        reloaded.color("accent.primary"),
        OpalineColor::new(225, 53, 255)
    );
    assert_eq!(reloaded.color("bg.base"), OpalineColor::new(18, 18, 24));
    assert_eq!(
        reloaded.style("keyword").fg,
        Some(OpalineColor::new(225, 53, 255))
    );
    assert!(reloaded.style("keyword").bold);
}

#[cfg(feature = "gradients")]
#[test]
fn to_toml_includes_gradients() {
    use opaline::Gradient;

    let theme = Theme::builder("Grad Test")
        .gradient(
            "primary",
            Gradient::new(vec![
                OpalineColor::new(225, 53, 255),
                OpalineColor::new(128, 255, 234),
            ]),
        )
        .build();

    let toml_str = theme.to_toml().expect("serialization succeeds");
    assert!(toml_str.contains("[gradients]"));
    assert!(toml_str.contains("#e135ff"));
    assert!(toml_str.contains("#80ffea"));

    let reloaded = load_from_str(&toml_str, None).expect("deserialization succeeds");
    let gradient = reloaded.get_gradient("primary").expect("gradient exists");
    assert_eq!(gradient.len(), 2);
}

#[test]
fn to_theme_file_preserves_palette() {
    let theme = Theme::builder("Palette Test")
        .palette("purple", OpalineColor::new(225, 53, 255))
        .palette("cyan", OpalineColor::new(128, 255, 234))
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .build();

    let theme_file = theme.to_theme_file();
    assert_eq!(
        theme_file.palette.get("purple"),
        Some(&"#e135ff".to_string())
    );
    assert_eq!(theme_file.palette.get("cyan"), Some(&"#80ffea".to_string()));
    assert_eq!(
        theme_file.tokens.get("accent.primary"),
        Some(&"#e135ff".to_string())
    );
}

#[test]
fn to_toml_style_all_modifiers() {
    let theme = Theme::builder("Modifiers")
        .style(
            "everything",
            OpalineStyle::fg(OpalineColor::new(255, 255, 255))
                .bold()
                .dim()
                .italic()
                .underline()
                .slow_blink()
                .rapid_blink()
                .reversed()
                .hidden()
                .crossed_out(),
        )
        .build();

    let toml_str = theme.to_toml().expect("serialization succeeds");
    let reloaded = load_from_str(&toml_str, None).expect("round-trip");
    let style = reloaded.style("everything");
    assert!(style.bold);
    assert!(style.dim);
    assert!(style.italic);
    assert!(style.underline);
    assert!(style.slow_blink);
    assert!(style.rapid_blink);
    assert!(style.reversed);
    assert!(style.hidden);
    assert!(style.crossed_out);
}

#[test]
fn to_toml_empty_theme() {
    let theme = Theme::builder("Empty").build();
    let toml_str = theme.to_toml().expect("serialization succeeds");
    assert!(toml_str.contains("name = \"Empty\""));
    let reloaded = load_from_str(&toml_str, None).expect("round-trip");
    assert_eq!(reloaded.meta.name, "Empty");
}

#[test]
fn save_to_file_creates_valid_toml() {
    let dir = std::env::temp_dir().join("opaline_test_export");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("test_theme.toml");

    let theme = Theme::builder("File Test")
        .token("accent.primary", OpalineColor::new(225, 53, 255))
        .build();

    theme.save_to_file(&path).expect("save succeeds");

    let reloaded = opaline::load_from_file(&path).expect("reload succeeds");
    assert_eq!(reloaded.meta.name, "File Test");
    assert_eq!(
        reloaded.color("accent.primary"),
        OpalineColor::new(225, 53, 255)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(feature = "builtin-themes")]
#[test]
fn builtin_theme_round_trips() {
    let theme = Theme::default();
    let toml_str = theme.to_toml().expect("serialization succeeds");
    let reloaded = load_from_str(&toml_str, None).expect("round-trip");

    assert_eq!(reloaded.meta.name, theme.meta.name);
    assert_eq!(
        reloaded.color("accent.primary"),
        theme.color("accent.primary")
    );
    assert_eq!(reloaded.style("keyword").bold, theme.style("keyword").bold);
}
