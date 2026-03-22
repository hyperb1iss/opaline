# egui Adapter

The `egui` feature maps Opaline themes onto [egui](https://www.egui.rs/)'s `Visuals` system — bringing token-based theming to immediate-mode GUI applications.

```toml
[dependencies]
opaline = { version = "0.2", features = ["egui"] }
```

## Color Conversion

`OpalineColor` converts to `egui::Color32`:

```rust
use opaline::OpalineColor;
use egui::Color32;

let color = OpalineColor::new(225, 53, 255);
let egui_color: Color32 = color.into();
// → Color32::from_rgb(225, 53, 255)
```

## Visuals Generation

Convert a full Opaline theme to egui `Visuals`:

```rust
use opaline::adapters::egui::to_egui_visuals;

let theme = opaline::Theme::default();
let visuals = to_egui_visuals(&theme);

// Apply in your egui app
ctx.set_visuals(visuals);
```

The function starts from `Visuals::dark()` or `Visuals::light()` based on the theme variant, then overrides all color properties. Non-color properties (corner radii, shadows, spacing) retain their sensible defaults.

### Token → Visuals Mapping

| Opaline Token | Visuals Field |
|--------------|---------------|
| `bg.base` | `panel_fill`, `widgets.noninteractive.bg_fill` |
| `bg.panel` | `window_fill`, `widgets.inactive.bg_fill` |
| `bg.highlight` | `faint_bg_color`, `widgets.hovered.bg_fill` |
| `bg.code` | `code_bg_color` |
| `bg.selection` | `selection.bg_fill`, `widgets.active.bg_fill` |
| `text.primary` | `override_text_color` |
| `text.secondary` | `widgets.inactive.fg_stroke` |
| `text.muted` | `widgets.noninteractive.fg_stroke` |
| `accent.primary` | `hyperlink_color`, `selection.stroke`, `widgets.hovered.fg_stroke` |
| `accent.secondary` | `widgets.open.bg_stroke` |
| `border.focused` | `widgets.hovered.bg_stroke` |
| `border.unfocused` | `window_stroke`, `widgets.noninteractive.bg_stroke` |
| `warning` | `warn_fg_color` |
| `error` | `error_fg_color` |

### Widget States

Each widget state uses a different layer of the theme:

| Widget State | Background | Foreground | Border |
|-------------|-----------|------------|--------|
| Noninteractive | `bg.base` | `text.muted` | `border.unfocused` |
| Inactive | `bg.panel` | `text.secondary` | `border.unfocused` |
| Hovered | `bg.highlight` | `accent.primary` | `border.focused` |
| Active | `bg.selection` | `accent.primary` | `accent.primary` |
| Open | `bg.highlight` | `text.primary` | `accent.secondary` |

## Runtime Theme Switching

```rust
use opaline::adapters::egui::to_egui_visuals;

fn switch_theme(ctx: &egui::Context, theme_name: &str) {
    let theme = opaline::load_by_name(theme_name).expect("valid theme");
    ctx.set_visuals(to_egui_visuals(&theme));
}
```

All 20 builtin themes work with egui — dark themes start from `Visuals::dark()`, light themes from `Visuals::light()`.
