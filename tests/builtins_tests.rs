use opaline::builtins;
use opaline::schema::ThemeVariant;
use opaline::OpalineColor;
use pretty_assertions::assert_eq;

#[test]
fn silkcircuit_neon_loads() {
    let theme = builtins::silkcircuit_neon();
    assert_eq!(theme.meta.name, "SilkCircuit Neon");
    assert_eq!(theme.meta.variant, ThemeVariant::Dark);
    assert_eq!(
        theme.meta.author.as_deref(),
        Some("hyperb1iss")
    );
}

#[test]
fn silkcircuit_neon_accent_colors() {
    let theme = builtins::silkcircuit_neon();
    // Electric Purple
    assert_eq!(
        theme.color("accent.primary"),
        OpalineColor::new(225, 53, 255)
    );
    // Neon Cyan
    assert_eq!(
        theme.color("accent.secondary"),
        OpalineColor::new(128, 255, 234)
    );
}

#[test]
fn silkcircuit_neon_has_all_semantic_tokens() {
    let theme = builtins::silkcircuit_neon();

    let required_tokens = [
        "text.primary",
        "text.secondary",
        "text.muted",
        "text.dim",
        "bg.base",
        "bg.panel",
        "bg.code",
        "bg.highlight",
        "accent.primary",
        "accent.secondary",
        "accent.tertiary",
        "accent.deep",
        "success",
        "error",
        "warning",
        "info",
        "git.staged",
        "git.modified",
        "git.untracked",
        "git.deleted",
        "diff.added",
        "diff.removed",
        "diff.hunk",
        "diff.context",
        "border.focused",
        "border.unfocused",
        "code.hash",
        "code.path",
        "code.keyword",
        "code.function",
        "code.string",
        "code.number",
        "code.comment",
        "code.type",
        "code.line_number",
        "mode.active",
        "mode.inactive",
        "mode.hover",
        "chat.user",
        "chat.iris",
    ];

    for token in &required_tokens {
        assert!(
            theme.has_token(token),
            "missing required token: {token}"
        );
    }
}

#[test]
fn silkcircuit_neon_has_expected_styles() {
    let theme = builtins::silkcircuit_neon();

    let expected_styles = [
        "keyword",
        "file_path",
        "commit_hash",
        "selected",
        "active_selected",
        "focused_border",
        "unfocused_border",
        "success_style",
        "error_style",
        "warning_style",
        "info_style",
        "dimmed",
        "muted",
        "inline_code",
        "git_staged",
        "git_modified",
        "diff_added",
        "diff_removed",
    ];

    for name in &expected_styles {
        assert!(
            theme.has_style(name),
            "missing expected style: {name}"
        );
    }
}

#[test]
fn silkcircuit_neon_keyword_style_is_bold_purple() {
    let theme = builtins::silkcircuit_neon();
    let kw = theme.style("keyword");
    assert_eq!(kw.fg, Some(OpalineColor::new(225, 53, 255)));
    assert!(kw.bold);
}

#[test]
fn silkcircuit_neon_has_gradients() {
    let theme = builtins::silkcircuit_neon();
    assert!(theme.has_gradient("primary"));
    assert!(theme.has_gradient("warm"));
    assert!(theme.has_gradient("aurora"));
}

#[test]
fn silkcircuit_neon_primary_gradient_endpoints() {
    let theme = builtins::silkcircuit_neon();
    // Purple -> Cyan
    assert_eq!(
        theme.gradient("primary", 0.0),
        OpalineColor::new(225, 53, 255)
    );
    assert_eq!(
        theme.gradient("primary", 1.0),
        OpalineColor::new(128, 255, 234)
    );
}

#[test]
fn load_by_name_known() {
    assert!(builtins::load_by_name("silkcircuit-neon").is_some());
    assert!(builtins::load_by_name("default").is_some());
}

#[test]
fn load_by_name_unknown() {
    assert!(builtins::load_by_name("nonexistent-theme").is_none());
}

#[test]
fn builtin_names_contains_neon() {
    let names = builtins::builtin_names();
    assert!(names.iter().any(|(id, _)| *id == "silkcircuit-neon"));
}

#[test]
fn list_available_themes_includes_builtins() {
    let themes = builtins::list_available_themes();
    assert!(!themes.is_empty());
    assert!(themes.iter().any(|t| t.name == "silkcircuit-neon" && t.builtin));
}

#[test]
fn default_theme_is_silkcircuit_neon() {
    let theme = opaline::Theme::default();
    assert_eq!(theme.meta.name, "SilkCircuit Neon");
}
