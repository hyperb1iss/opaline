# Gradients

Opaline's gradient system provides smooth color transitions between any number of stops. Gradients are perfect for progress bars, status indicators, decorative headers, and any UI element that benefits from color flow.

::: tip
The `gradients` feature is enabled by default. Disable it with `default-features = false` if you don't need gradients.
:::

## Defining Gradients

In TOML, gradients are lists of palette color names:

```toml
[palette]
purple = "#e135ff"
cyan = "#80ffea"
coral = "#ff6ac1"
green = "#50fa7b"

[gradients]
primary = ["purple", "cyan"]
warm = ["coral", "purple"]
aurora = ["purple", "cyan", "green", "coral", "purple"]
```

## Sampling

The `Gradient::at(t)` method samples a color at position `t` (0.0 to 1.0):

```rust
let theme = opaline::Theme::default();

// Endpoints
let start = theme.gradient("primary", 0.0); // first stop color
let end = theme.gradient("primary", 1.0);   // last stop color

// Midpoint — interpolated
let mid = theme.gradient("primary", 0.5);
```

Values outside `[0.0, 1.0]` are clamped.

## Generating Color Sequences

Use `generate(n)` to produce evenly-spaced colors:

```rust
let gradient = opaline::Gradient::new(vec![
    OpalineColor::new(225, 53, 255),
    OpalineColor::new(128, 255, 234),
]);

let colors = gradient.generate(10); // 10 evenly-spaced colors
assert_eq!(colors.len(), 10);
```

## Required Gradients

Every builtin theme defines these 5 gradients:

| Gradient | Purpose |
|----------|---------|
| `primary` | Main accent gradient (accent.primary → accent.secondary) |
| `warm` | Warm tones gradient |
| `success_gradient` | Success state gradient |
| `error_gradient` | Error state gradient |
| `aurora` | Multi-stop decorative gradient (5 stops) |

## Ratatui Integration

With the `ratatui` feature, gradients become Ratatui spans directly.

### High-Level: Theme Methods

The simplest way to render gradient text — no extra imports:

```rust
let theme = opaline::Theme::default();

// Per-character gradient coloring → Line
let line = theme.gradient_text("aurora", "Hello, Opaline!");
```

### Low-Level: Free Functions

For maximum control, use the free functions with a `&Gradient`:

```rust
use opaline::{Theme, gradient_spans, gradient_text_line, gradient_bar};

let theme = Theme::default();
let gradient = theme.get_gradient("primary").unwrap();

// Vec<Span> — each character colored along the gradient
let spans = gradient_spans("Status: Online", gradient);

// Line — gradient-colored text
let line = gradient_text_line("Loading...", gradient);

// Line — repeated block characters for progress bars
let bar = gradient_bar(40, '█', gradient);
```

## Building Gradients Programmatically

```rust
use opaline::{Gradient, OpalineColor};

// From a Vec of colors
let gradient = Gradient::new(vec![
    OpalineColor::new(225, 53, 255),
    OpalineColor::new(128, 255, 234),
    OpalineColor::new(80, 250, 123),
]);

// Fallible construction
let result = Gradient::try_new(vec![]); // returns Err
```

## Direct Gradient Access

```rust
let theme = opaline::Theme::default();

// Get the Gradient object for manual use
if let Some(grad) = theme.get_gradient("primary") {
    let colors = grad.generate(20);
    let stops = grad.stops();
    let len = grad.len();
}

// Check existence
assert!(theme.has_gradient("primary"));

// List all gradient names
let names = theme.gradient_names();
```
