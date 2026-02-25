# App-Level Derivation

Most apps need tokens beyond what a generic theme provides — hover states, selection backgrounds, sidebar accents. Rather than requiring every theme author to define these, Opaline lets you **derive** app-specific tokens from the theme's existing palette.

## The Pattern

Load a theme, inject your derived tokens, then activate it:

```rust
opaline::load_theme_by_name_with("catppuccin-mocha", |theme| {
    let primary = theme.color("accent.primary");

    // App-specific tokens derived from the theme palette
    theme.register_default_token("sidebar.bg", primary.darken(0.85));
    theme.register_default_token("sidebar.hover", primary.darken(0.75));
    theme.register_default_token("tab.active", primary.lighten(0.1));

    // App-specific styles
    theme.register_default_style("sidebar_item", OpalineStyle::fg(primary).bold());
});
```

After this call, the theme is the active global theme with your derived tokens available via `theme.color("sidebar.bg")` like any other token.

## Token Registration

### Default Registration

`register_default_token` and `register_default_style` use entry semantics — **TOML-defined values win**. This lets theme authors override your derivations when they want to:

```rust
// Only inserted if the theme doesn't already define "sidebar.bg"
theme.register_default_token("sidebar.bg", primary.darken(0.85));

// Only inserted if the theme doesn't already define "sidebar_item"
theme.register_default_style("sidebar_item", OpalineStyle::fg(primary));
```

This is the recommended approach — it respects theme author intent.

### Forced Registration

`register_token` and `register_style` unconditionally overwrite:

```rust
// Always overwrites, even if the theme defines this token
theme.register_token("sidebar.bg", primary.darken(0.85));

// Always overwrites the style
theme.register_style("sidebar_item", OpalineStyle::fg(primary));
```

Use this sparingly — only when your app requires a specific derived value regardless of theme authoring.

## Global State Functions

### `load_theme_by_name_with`

Requires: `global-state` + `builtin-themes`

Load a builtin or discovered theme, run derivation, then set as the active global theme:

```rust
use opaline::load_theme_by_name_with;

load_theme_by_name_with("dracula", |theme| {
    // Derive app tokens here
})?;
```

### `load_theme_by_name_for_app_with`

Requires: `global-state` + `builtin-themes` + `discovery`

Same as above, but also searches app-specific discovery paths (`~/.config/<app>/themes/`):

```rust
use opaline::load_theme_by_name_for_app_with;

load_theme_by_name_for_app_with("custom-theme", "myapp", |theme| {
    // Derive app tokens here
})?;
```

## Defining a Derive Function

For apps with many derived tokens, extract the derivation into a named function:

```rust
fn derive_tokens(theme: &mut opaline::Theme) {
    let primary = theme.color("accent.primary");
    let bg = theme.color("bg.base");

    // Navigation
    theme.register_default_token("nav.bg", bg.darken(0.1));
    theme.register_default_token("nav.hover", primary.darken(0.7));
    theme.register_default_token("nav.active", primary);

    // Status bar
    theme.register_default_token("status.bg", bg.lighten(0.05));
    theme.register_default_token("status.text", theme.color("text.secondary"));
}

// Use it everywhere you load themes
opaline::load_theme_by_name_with("nord", derive_tokens)?;
```

## With the ThemeSelector Widget

The [ThemeSelector widget](./theme-selector.md) accepts a derivation callback so live preview includes your derived tokens:

```rust
let state = ThemeSelectorState::with_current_selected()
    .with_derive(derive_tokens);
```

Every theme preview will have your app-specific tokens applied, giving users an accurate look at what each theme will actually look like in your app.
