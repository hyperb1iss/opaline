# iced Adapter

The `iced` feature maps Opaline themes onto [iced](https://iced.rs/)'s `Palette` and `Custom` theme types, so a single TOML theme drives both your terminal UI and your desktop GUI.

```toml
[dependencies]
opaline = { version = "0.4", features = ["iced"] }
```

## Color Conversion

`OpalineColor` converts to `iced::Color`:

```rust
use opaline::OpalineColor;
use iced::Color;

let color = OpalineColor::new(225, 53, 255);
let iced_color: Color = color.into();
// → Color::from_rgb8(225, 53, 255)
```

## Palette Generation

Convert a full Opaline theme to an iced `Palette`:

```rust
use opaline::adapters::iced::to_iced_palette;

let theme = opaline::Theme::default();
let palette = to_iced_palette(&theme);
```

### Token → Palette Mapping

| Opaline Token    | Palette Field |
| ---------------- | ------------- |
| `bg.base`        | `background`  |
| `text.primary`   | `text`        |
| `accent.primary` | `primary`     |
| `success`        | `success`     |
| `warning`        | `warning`     |
| `error`          | `danger`      |

iced derives the rest of its widget colors (weak, strong, hover variants) from these six slots.

## Custom Theme

`to_iced_custom` wraps the palette in an iced `Custom` theme named after the Opaline theme. Drop it into `iced::Theme::Custom`:

```rust
use std::sync::Arc;
use iced::Theme;
use opaline::adapters::iced::to_iced_custom;

let theme = opaline::Theme::default();
let custom = to_iced_custom(&theme);
let iced_theme = Theme::Custom(Arc::new(custom));
```

Return `iced_theme` from your application's `theme` method and every widget picks it up.

If you need the derived tints directly, `to_iced_extended` returns the `Extended` palette that iced generates from the base palette.

## Runtime Theme Switching

```rust
use std::sync::Arc;
use iced::Theme;
use opaline::adapters::iced::to_iced_custom;

fn switch_theme(theme_name: &str) -> Theme {
    let theme = opaline::load_by_name(theme_name).expect("valid theme");
    Theme::Custom(Arc::new(to_iced_custom(&theme)))
}
```

All 39 builtin themes work with iced. iced decides between its dark and light widget styling from the lightness of `background`, which agrees with each builtin theme's declared variant.
