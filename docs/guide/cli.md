# CLI Adapter

The `cli` feature enables integration with the [`colored`](https://crates.io/crates/colored) crate for non-TUI terminal output — scripts, formatters, log output, and CLI tools.

```toml
[dependencies]
opaline = { version = "0.2", features = ["cli"] }
```

## ThemeCliExt

```rust
use opaline::{Theme, ThemeCliExt};

let theme = Theme::default();

// Get an RGB tuple for manual coloring
let rgb = theme.cli_rgb("accent.primary");
println!("{rgb:?}");

// Apply a token color to a string
let colored_text = theme.cli_colored("important text", "accent.primary");
println!("{colored_text}");
```

## ColoredExt

Apply Opaline colors directly to `colored` strings:

```rust
use opaline::{OpalineColor, ColoredExt};

let color = OpalineColor::new(225, 53, 255);
let fg_text = "electric purple".theme_fg(color);
let bg_text = "electric purple".theme_bg(color);
println!("{fg_text}");
println!("{bg_text}");
```

## Gradient Strings

With `cli` + `gradients`:

```rust
use opaline::{Theme, ThemeCliExt};

let theme = Theme::default();
let rainbow = theme.cli_gradient("Gradient text in the terminal!", "aurora");
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
