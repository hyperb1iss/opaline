use opaline::OpalineColor;
use opaline::builtins;
use opaline::schema::ThemeVariant;
use pretty_assertions::assert_eq;
use std::fs;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[cfg(all(
    feature = "builtin-themes",
    feature = "discovery",
    feature = "global-state"
))]
fn global_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().expect("lock")
}

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
    "bg.selection",
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
fn list_available_themes_keeps_builtin_ids_unique() {
    let themes = builtins::list_available_themes();

    for &(id, _) in builtins::builtin_names() {
        let visible_count = themes.iter().filter(|theme| theme.name == id).count();
        assert_eq!(
            visible_count, 1,
            "expected exactly one visible theme entry for builtin id {id}"
        );
    }
}

#[test]
fn default_theme_is_silkcircuit_neon() {
    let theme = opaline::Theme::default();
    assert_eq!(theme.meta.name, "SilkCircuit Neon");
}

#[test]
fn theme_info_load_prefers_path_over_builtin_id() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("opaline-theme-info-{unique}"));
    fs::create_dir_all(&temp_dir).expect("create temp dir");

    let path = temp_dir.join("dracula.toml");
    fs::write(
        &path,
        r##"
[meta]
name = "Shadow Dracula"
variant = "dark"
author = "local"

[palette]
shadow = "#010203"
"##,
    )
    .expect("write theme file");

    let info = builtins::ThemeInfo {
        name: "dracula".to_string(),
        display_name: "Shadow Dracula".to_string(),
        variant: ThemeVariant::Dark,
        author: "local".to_string(),
        description: String::new(),
        builtin: false,
        path: Some(path),
    };

    let theme = info.load().expect("theme loads");
    assert_eq!(theme.meta.name, "Shadow Dracula");
}

#[cfg(all(
    feature = "builtin-themes",
    feature = "discovery",
    feature = "global-state"
))]
#[test]
fn discovered_theme_shadowing_builtin_wins_in_loader_and_listing() {
    let _guard = global_lock();
    let previous = opaline::current();
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!("opaline-config-{unique}"));
    fs::create_dir_all(&temp_root).expect("create temp config root");
    let base_dir = temp_root.join("opaline").join("themes");
    let app_dir = temp_root.join("opaline-local").join("themes");

    fs::create_dir_all(&base_dir).expect("create base theme dir");
    fs::create_dir_all(&app_dir).expect("create app theme dir");

    let builtin = builtins::load_by_name("dracula").expect("builtin loads");
    assert_eq!(builtin.meta.name, "Dracula");

    fs::write(
        base_dir.join("dracula.toml"),
        r##"
[meta]
name = "Shadow Dracula"
variant = "dark"
author = "local"

[palette]
shadow = "#010203"
"##,
    )
    .expect("write shadowing theme");

    fs::write(
        app_dir.join("opaline-local.toml"),
        r##"
[meta]
name = "Opaline Local"
variant = "light"
author = "local"

[palette]
accent = "#abcdef"
"##,
    )
    .expect("write app theme");

    opaline::load_theme_by_name_in_dirs("dracula", [base_dir.clone()]).expect("custom theme loads");
    assert_eq!(opaline::current().meta.name, "Shadow Dracula");

    let all = builtins::list_available_themes_in_dirs([base_dir.clone(), app_dir.clone()]);
    let dracula = all
        .iter()
        .find(|theme| theme.name == "dracula")
        .expect("dracula theme present");
    assert!(!dracula.builtin);
    assert_eq!(dracula.display_name, "Shadow Dracula");
    assert!(dracula.path.is_some());

    let app_themes = builtins::list_available_themes_in_dirs([app_dir.clone()]);
    assert!(
        app_themes
            .iter()
            .any(|theme| theme.name == "opaline-local" && theme.display_name == "Opaline Local")
    );

    let _ = fs::remove_file(base_dir.join("dracula.toml"));
    let _ = fs::remove_file(app_dir.join("opaline-local.toml"));

    opaline::set_theme((*previous).clone());
}
