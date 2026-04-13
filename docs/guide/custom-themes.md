# Custom Themes

Opaline makes it easy for users to create their own themes. Any valid TOML file with the right structure becomes a theme.

## Theme File Template

Start from this template:

```toml
[meta]
name = "My Custom Theme"
author = "your-name"
variant = "dark"        # or "light"
version = "1.0"
description = "A short description of your theme"

[palette]
# Define your raw colors here
bg = "#1a1a2e"
fg = "#e0e0e0"
accent = "#e135ff"
secondary = "#80ffea"
muted = "#6a6a7a"
red = "#ff6363"
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#80bfff"
orange = "#ffb86c"

[tokens]
# Map semantic names to palette colors
"text.primary" = "fg"
"text.secondary" = "fg"       # can duplicate for simplicity
"text.muted" = "muted"
"text.dim" = "muted"

"bg.base" = "bg"
"bg.panel" = "bg"
"bg.code" = "bg"
"bg.highlight" = "bg"
"bg.selection" = "bg"

"accent.primary" = "accent"
"accent.secondary" = "secondary"
"accent.tertiary" = "blue"
"accent.deep" = "accent"

success = "green"
error = "red"
warning = "yellow"
info = "blue"

"border.focused" = "accent"
"border.unfocused" = "muted"

"code.keyword" = "accent"
"code.function" = "secondary"
"code.string" = "green"
"code.number" = "orange"
"code.comment" = "muted"
"code.type" = "yellow"
"code.line_number" = "muted"

[styles]
keyword = { fg = "accent.primary", bold = true }
line_number = { fg = "code.line_number" }
selected = { fg = "accent.secondary", bg = "bg.highlight" }
active_selected = { fg = "accent.primary", bg = "bg.highlight", bold = true }
focused_border = { fg = "border.focused" }
unfocused_border = { fg = "border.unfocused" }
success_style = { fg = "success" }
error_style = { fg = "error" }
warning_style = { fg = "warning" }
info_style = { fg = "info" }
dimmed = { fg = "text.dim" }
muted = { fg = "text.muted" }
inline_code = { fg = "success", bg = "bg.code" }

[gradients]
primary = ["accent", "secondary"]
warm = ["orange", "yellow"]
success_gradient = ["green", "secondary"]
error_gradient = ["red", "orange"]
aurora = ["accent", "secondary", "green", "blue", "accent"]
```

## Loading Custom Themes

```rust
// From a file
let theme = opaline::load_from_file("~/.config/myapp/themes/custom.toml")?;

// From a string (e.g., embedded or fetched)
let toml_str = std::fs::read_to_string("theme.toml")?;
let theme = opaline::load_from_str(&toml_str, None)?;
```

## Theme Discovery

With the `discovery` feature, Opaline can scan standard directories for user themes:

```rust
// Get theme directories for your app
let dirs = opaline::app_theme_dirs("myapp");
// → ~/.config/myapp/themes/

// List discoverable themes for a specific app
let themes = opaline::builtins::list_available_themes_for_app("myapp");

// Scan all theme directories
let dirs = opaline::theme_dirs();
```

If a custom theme file uses the same id as a builtin, the file-backed theme wins in discovery and name-based loading. That keeps app-specific themes predictable when users override a shipped theme id.

For application-specific semantics like git status colors, diff colors, or view-mode indicators, derive extra tokens and styles in the consuming app instead of baking them into the core theme contract.

## Validation

The strict resolver catches issues at load time:

- **Missing palette color**: a token references a color that doesn't exist
- **Circular reference**: tokens form a cycle (`a → b → a`)
- **Invalid hex**: a palette value isn't a valid hex color

If your theme loads without error, it's valid. For builtin-level quality, ensure it defines all required tokens, 13 required styles, and 5 required gradients.

## Tips

- **Start from an existing theme.** Copy a builtin TOML and modify colors.
- **Use descriptive palette names.** `sumi_ink3` is better than `bg3` for readability.
- **Test with both light and dark terminals.** Set `variant` correctly.
- **Keep gradients harmonious.** Adjacent stops should blend smoothly.
