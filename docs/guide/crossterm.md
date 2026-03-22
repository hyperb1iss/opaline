# Crossterm Adapter

The `crossterm` feature enables direct terminal styling via [crossterm](https://crates.io/crates/crossterm) — ideal for apps that use crossterm without ratatui for raw terminal manipulation.

```toml
[dependencies]
opaline = { version = "0.2", features = ["crossterm"] }
```

## From Impls

Opaline types convert directly to crossterm types:

```rust
use opaline::OpalineColor;
use crossterm::style::Color;

let color = OpalineColor::new(225, 53, 255);
let ct_color: Color = color.into();
// → Color::Rgb { r: 225, g: 53, b: 255 }
```

`OpalineStyle` converts to `ContentStyle` with all 9 modifiers mapped to crossterm `Attribute` flags:

```rust
use opaline::{Theme, OpalineStyle};
use crossterm::style::ContentStyle;

let theme = Theme::default();
let style: ContentStyle = theme.style("keyword").into();
```

## Theme Helper

Apply a named style to content directly:

```rust
use opaline::Theme;

let theme = Theme::default();
let styled = theme.crossterm_styled("keyword", "fn main");
print!("{styled}");
```

## Gradient Rendering

With `crossterm` + `gradients`:

```rust
use opaline::adapters::crossterm::{gradient_styled, gradient_bar};

let theme = opaline::Theme::default();
if let Some(gradient) = theme.get_gradient("primary") {
    // Per-character gradient text
    for styled in gradient_styled("Hello, world!", gradient) {
        print!("{styled}");
    }

    // Gradient progress bar
    for styled in gradient_bar(40, '█', gradient) {
        print!("{styled}");
    }
}
```

## When to Use Crossterm vs Ratatui

| Use Case | Adapter |
|----------|---------|
| Full TUI app with widgets | `ratatui` |
| Raw terminal manipulation | `crossterm` |
| Custom rendering loop | `crossterm` |
| Alternate screen apps without ratatui | `crossterm` |
