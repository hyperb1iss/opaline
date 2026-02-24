use opaline::OpalineColor;
use opaline::builtins;
use opaline::schema::ThemeVariant;
use pretty_assertions::assert_eq;

// ── SilkCircuit Neon (default) ───────────────────────────────────────────

#[test]
fn silkcircuit_neon_loads() {
    let theme = builtins::silkcircuit_neon();
    assert_eq!(theme.meta.name, "SilkCircuit Neon");
    assert_eq!(theme.meta.variant, ThemeVariant::Dark);
    assert_eq!(theme.meta.author.as_deref(), Some("hyperb1iss"));
}

#[test]
fn silkcircuit_neon_accent_colors() {
    let theme = builtins::silkcircuit_neon();
    assert_eq!(
        theme.color("accent.primary"),
        OpalineColor::new(225, 53, 255)
    );
    assert_eq!(
        theme.color("accent.secondary"),
        OpalineColor::new(128, 255, 234)
    );
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
    assert_eq!(
        theme.gradient("primary", 0.0),
        OpalineColor::new(225, 53, 255)
    );
    assert_eq!(
        theme.gradient("primary", 1.0),
        OpalineColor::new(128, 255, 234)
    );
}

// ── Every builtin loads without panic ────────────────────────────────────

#[test]
fn all_builtins_load() {
    for &(id, display) in builtins::builtin_names() {
        let theme = builtins::load_by_name(id)
            .unwrap_or_else(|| panic!("builtin '{id}' ({display}) failed to load"));
        assert_eq!(theme.meta.name, display, "display name mismatch for {id}");
    }
}

#[test]
fn builtin_count_is_20() {
    assert_eq!(builtins::builtin_names().len(), builtins::BUILTIN_COUNT);
    assert_eq!(builtins::BUILTIN_COUNT, 20);
}

// ── Token contract: every builtin has the required semantic tokens ────────

const REQUIRED_TOKENS: &[&str] = &[
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
];

#[test]
fn all_builtins_have_required_tokens() {
    for &(id, _) in builtins::builtin_names() {
        let theme = builtins::load_by_name(id).expect("loads");
        for &token in REQUIRED_TOKENS {
            assert!(
                theme.has_token(token),
                "theme '{id}' missing required token: {token}"
            );
        }
    }
}

const REQUIRED_STYLES: &[&str] = &[
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

#[test]
fn all_builtins_have_required_styles() {
    for &(id, _) in builtins::builtin_names() {
        let theme = builtins::load_by_name(id).expect("loads");
        for &style in REQUIRED_STYLES {
            assert!(
                theme.has_style(style),
                "theme '{id}' missing required style: {style}"
            );
        }
    }
}

const REQUIRED_GRADIENTS: &[&str] = &[
    "primary",
    "warm",
    "success_gradient",
    "error_gradient",
    "aurora",
];

#[test]
fn all_builtins_have_required_gradients() {
    for &(id, _) in builtins::builtin_names() {
        let theme = builtins::load_by_name(id).expect("loads");
        for &gradient in REQUIRED_GRADIENTS {
            assert!(
                theme.has_gradient(gradient),
                "theme '{id}' missing required gradient: {gradient}"
            );
        }
    }
}

// ── Variant correctness ──────────────────────────────────────────────────

#[test]
fn light_themes_marked_correctly() {
    let light_ids = ["catppuccin-latte", "solarized-light", "silkcircuit-dawn"];
    for id in &light_ids {
        let theme = builtins::load_by_name(id).expect("loads");
        assert!(
            theme.is_light(),
            "theme '{id}' should be ThemeVariant::Light"
        );
    }
}

#[test]
fn dark_themes_marked_correctly() {
    let dark_ids = [
        "silkcircuit-neon",
        "silkcircuit-soft",
        "silkcircuit-glow",
        "silkcircuit-vibrant",
        "catppuccin-mocha",
        "dracula",
        "nord",
        "tokyo-night",
        "gruvbox-dark",
        "one-dark",
    ];
    for id in &dark_ids {
        let theme = builtins::load_by_name(id).expect("loads");
        assert!(theme.is_dark(), "theme '{id}' should be ThemeVariant::Dark");
    }
}

// ── Registry ─────────────────────────────────────────────────────────────

#[test]
fn load_by_name_default_alias() {
    let default = builtins::load_by_name("default").expect("default loads");
    let neon = builtins::load_by_name("silkcircuit-neon").expect("neon loads");
    assert_eq!(default.meta.name, neon.meta.name);
}

#[test]
fn load_by_name_unknown() {
    assert!(builtins::load_by_name("nonexistent-theme").is_none());
}

#[test]
fn list_available_themes_includes_all_builtins() {
    let themes = builtins::list_available_themes();
    assert_eq!(
        themes.iter().filter(|t| t.builtin).count(),
        builtins::BUILTIN_COUNT
    );
}

#[test]
fn default_theme_is_silkcircuit_neon() {
    let theme = opaline::Theme::default();
    assert_eq!(theme.meta.name, "SilkCircuit Neon");
}
