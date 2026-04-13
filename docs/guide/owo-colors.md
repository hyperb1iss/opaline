# owo-colors Adapter

The `owo-colors` feature enables zero-allocation terminal coloring via [owo-colors](https://crates.io/crates/owo-colors), a modern, lightweight alternative to the `colored` crate.

```toml
[dependencies]
opaline = { version = "0.4", features = ["owo-colors"] }
```

## Style Conversion

`OpalineStyle` converts to `owo_colors::Style`:

```rust
use opaline::{OpalineStyle, OpalineColor};
use owo_colors::{OwoColorize, Style};

let style: Style = OpalineStyle::fg(OpalineColor::new(225, 53, 255))
    .bold()
    .into();

println!("{}", "keyword".style(style));
```

## OwoThemeExt

The extension trait provides convenient themed styling:

```rust
use opaline::{Theme, OwoThemeExt};
use owo_colors::OwoColorize;

let theme = Theme::default();

// Full style from a named style
let kw_style = theme.owo_style("keyword");
println!("{}", "fn".style(kw_style));

// Token color as foreground
let fg_style = theme.owo_fg("accent.primary");
println!("{}", "accent text".style(fg_style));

// Token color as background
let bg_style = theme.owo_bg("bg.highlight");
println!("{}", "highlighted".style(bg_style));
```

## Gradient Strings

With `owo-colors` + `gradients`:

```rust
use opaline::adapters::owo_colors::gradient_string;

let theme = opaline::Theme::default();
if let Some(gradient) = theme.get_gradient("aurora") {
    let output = gradient_string("Rainbow gradient text!", gradient);
    println!("{output}");
}
```

## owo-colors vs colored

| Feature        | `owo-colors`    | `colored`                 |
| -------------- | --------------- | ------------------------- |
| Allocation     | Zero-alloc      | Allocates `ColoredString` |
| API style      | Extension trait | Extension trait           |
| `no_std`       | Supported       | No                        |
| Runtime styles | `Style` builder | Per-call methods          |
| Ecosystem      | Modern, growing | Established, widely used  |

Both adapters can coexist. Enable `cli` and `owo-colors` simultaneously if needed.
