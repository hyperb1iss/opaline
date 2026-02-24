# CLI Adapter

The `cli` feature enables integration with the [`colored`](https://crates.io/crates/colored) crate for non-TUI terminal output — scripts, formatters, log output, and CLI tools.

```toml
[dependencies]
opaline = { version = "0.1", features = ["cli"] }
```

## ThemeCliExt

```rust
use opaline::{Theme, ThemeCliExt};

let theme = Theme::default();

// Apply a token color to a string
let colored_text = theme.colorize("accent.primary", "important text");
println!("{colored_text}");

// Apply a named style
let styled = theme.style_text("keyword", "fn");
println!("{styled}");
```

## ColoredExt

Apply Opaline colors directly to `colored` strings:

```rust
use opaline::{OpalineColor, ColoredExt};
use colored::Colorize;

let color = OpalineColor::new(225, 53, 255);
let text = "electric purple".opaline_fg(color);
println!("{text}");
```

## Gradient Strings

With `cli` + `gradients`:

```rust
use opaline::{Theme, gradient_string};

let theme = Theme::default();
let rainbow = gradient_string(&theme, "aurora", "Gradient text in the terminal!");
println!("{rainbow}");
```

## When to Use CLI vs Ratatui

| Use Case | Adapter |
|----------|---------|
| Full TUI app (alternate screen) | `ratatui` |
| CLI tool output | `cli` |
| Log formatting | `cli` |
| Simple scripts | `cli` |
| Interactive terminal UI | `ratatui` |
