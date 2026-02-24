# Styles & Modifiers

Styles combine a foreground color, background color, and text modifiers into a single composable unit.

## OpalineStyle

The core style type:

```rust
use opaline::OpalineStyle;

// Foreground only
let style = OpalineStyle::fg(OpalineColor::new(225, 53, 255));

// With modifiers
let bold_keyword = OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold();

// Full builder
let fancy = OpalineStyle::new()
    .with_fg(OpalineColor::new(225, 53, 255))
    .with_bg(OpalineColor::new(18, 18, 24))
    .bold()
    .italic();
```

## Modifiers

Opaline supports all 9 terminal text modifiers:

| Modifier | Method | TOML Key |
|----------|--------|----------|
| **Bold** | `.bold()` | `bold = true` |
| **Italic** | `.italic()` | `italic = true` |
| **Underline** | `.underline()` | `underline = true` |
| **Dim** | `.dim()` | `dim = true` |
| **Strikethrough** | `.strikethrough()` | `strikethrough = true` |
| **Reversed** | `.reversed()` | `reversed = true` |
| **Hidden** | `.hidden()` | `hidden = true` |
| **Rapid Blink** | `.rapid_blink()` | `rapid_blink = true` |
| **Slow Blink** | `.slow_blink()` | `slow_blink = true` |

## Defining Styles in TOML

Styles reference tokens for their colors:

```toml
[styles]
keyword = { fg = "accent.primary", bold = true }
selected = { fg = "accent.secondary", bg = "bg.highlight" }
error_style = { fg = "error", bold = true, underline = true }
dimmed = { fg = "text.dim" }
inline_code = { fg = "success", bg = "bg.code" }
```

## Required Styles

Every builtin theme must define these 18 styles:

| Style | Purpose |
|-------|---------|
| `keyword` | Language keywords |
| `file_path` | File paths |
| `commit_hash` | Git commit hashes |
| `selected` | Selected item |
| `active_selected` | Active + selected item |
| `focused_border` | Focused panel border |
| `unfocused_border` | Unfocused panel border |
| `success_style` | Success state |
| `error_style` | Error state |
| `warning_style` | Warning state |
| `info_style` | Info state |
| `dimmed` | Dimmed/subtle text |
| `muted` | Muted text |
| `inline_code` | Inline code snippets |
| `git_staged` | Staged git changes |
| `git_modified` | Modified git files |
| `diff_added` | Diff added lines |
| `diff_removed` | Diff removed lines |

## Accessing Styles

```rust
let theme = opaline::Theme::default();

// Get a style (returns Default if missing)
let kw = theme.style("keyword");
assert!(kw.bold);

// Strict lookup
if let Some(style) = theme.try_style("keyword") {
    // ...
}

// Check existence
assert!(theme.has_style("keyword"));

// List all style names
let names = theme.style_names();
```

## Merging Styles

Styles can be merged — the overlay's non-default fields override the base:

```rust
let base = OpalineStyle::fg(OpalineColor::new(255, 255, 255));
let overlay = OpalineStyle::new().bold().italic();
let merged = base.merge(&overlay);

// merged has fg from base + bold/italic from overlay
```

This is useful for building compound styles from simpler components.
