# Quick Start

This guide gets you from zero to themed Ratatui rendering in under 5 minutes.

## Load a Theme

Every Opaline journey starts with a `Theme`:

```rust
use opaline::Theme;

// Default theme (SilkCircuit Neon)
let theme = Theme::default();

// Or pick a specific builtin
let theme = opaline::load_by_name("catppuccin-mocha").unwrap();

// Or load from a TOML file
let theme = opaline::load_from_file("my-theme.toml").unwrap();
```

## Access Colors and Styles

Themes provide **semantic access** — you ask for what something means, not what color it is:

```rust
// Get a resolved color by token name
let accent = theme.color("accent.primary");
let bg = theme.color("bg.base");

// Get a composed style (fg + bg + modifiers)
let keyword_style = theme.style("keyword");
assert!(keyword_style.bold);

// Check if tokens/styles exist
assert!(theme.has_token("accent.primary"));
assert!(theme.has_style("keyword"));
```

## Render with Ratatui

The `ratatui` feature (enabled by default) gives you seamless integration:

```rust
use opaline::Theme;
use ratatui::text::{Span, Line};

let theme = Theme::default();

// Create styled ratatui types directly — no trait import needed
let styled_span = theme.span("keyword", "fn");
let styled_line = theme.line("keyword", "let x = 42;");

// theme.style() works directly with widget methods via Into<Style>
let block = ratatui::widgets::Block::bordered()
    .style(theme.style("keyword"));

// For Style::fg()/bg(), convert colors explicitly
let color: ratatui::style::Color = theme.color("accent.primary").into();
```

## Use Gradients

Gradients produce smooth color transitions — great for progress bars and decorative elements:

```rust
use opaline::{Theme, gradient_bar};

let theme = Theme::default();

// Sample a single color from a gradient
let color = theme.gradient("primary", 0.5); // midpoint of primary gradient

// High-level: gradient-colored text as a Line
let line = theme.gradient_text("aurora", "Hello, Opaline!");

// Low-level: gradient progress bar from a &Gradient
let grad = theme.get_gradient("primary").unwrap();
let bar = gradient_bar(40, '█', grad); // 40 chars wide
```

## Build Themes Programmatically

Skip TOML entirely with `ThemeBuilder`:

```rust
use opaline::{Theme, OpalineColor, OpalineStyle};

let theme = Theme::builder("My Custom Theme")
    .author("me")
    .variant(opaline::ThemeVariant::Dark)
    .token("accent.primary", OpalineColor::new(255, 100, 200))
    .token("bg.base", OpalineColor::new(18, 18, 24))
    .style("keyword", OpalineStyle::fg(OpalineColor::new(255, 100, 200)).bold())
    .build();
```

## Next Steps

- [Theme System](../guide/themes) — Understand the resolution pipeline
- [Token Reference](../reference/tokens) — All 40+ semantic tokens
- [Theme Gallery](../themes/) — Browse all 20 builtin themes
