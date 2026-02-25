# Color Manipulation

`OpalineColor` provides methods for deriving new colors from existing ones — darkening, lightening, and desaturating. These are especially useful for [app-level token derivation](./derivation.md), where you compute UI-specific colors from a theme's palette.

## Methods

All methods take an `amount` parameter clamped to `[0.0, 1.0]` and return a new `OpalineColor`.

### `darken(amount)`

Mix toward black. `0.0` = unchanged, `1.0` = pure black.

```rust
let primary = theme.color("accent.primary");
let muted = primary.darken(0.3);      // 30% toward black
let shadow = primary.darken(0.7);     // deep shadow
```

### `lighten(amount)`

Mix toward white. `0.0` = unchanged, `1.0` = pure white.

```rust
let primary = theme.color("accent.primary");
let highlight = primary.lighten(0.2); // subtle highlight
let washed = primary.lighten(0.8);    // nearly white
```

### `desaturate(amount)`

Mix toward the luminance-equivalent gray (BT.601 weights: R×0.299 + G×0.587 + B×0.114). `0.0` = unchanged, `1.0` = fully gray.

```rust
let primary = theme.color("accent.primary");
let muted = primary.desaturate(0.5);  // half-saturation
let gray = primary.desaturate(1.0);   // pure luminance gray
```

## Constants

Two common color constants are available as associated constants:

```rust
use opaline::OpalineColor;

OpalineColor::BLACK  // OpalineColor { r: 0, g: 0, b: 0 }
OpalineColor::WHITE  // OpalineColor { r: 255, g: 255, b: 255 }
```

These are what `darken()` and `lighten()` mix toward internally — they all delegate to `lerp()`.

## Chaining

Methods return `Self`, so you can chain them:

```rust
let subtle = theme.color("accent.primary")
    .desaturate(0.3)
    .darken(0.1);
```

## Use with Token Derivation

Color manipulation shines when computing app-specific tokens from a theme's core palette:

```rust
opaline::load_theme_by_name_with("catppuccin-mocha", |theme| {
    let primary = theme.color("accent.primary");

    theme.register_default_token("bg.hover", primary.darken(0.8));
    theme.register_default_token("bg.selected", primary.darken(0.7));
    theme.register_default_token("text.muted", theme.color("text.primary").desaturate(0.4));
});
```

See [App-Level Derivation](./derivation.md) for the full pattern.
