# ThemeBuilder

`ThemeBuilder` lets you construct themes programmatically without TOML files. This is useful for:

- Testing (build minimal themes for unit tests)
- Dynamic themes (generate themes at runtime)
- Embedded themes (themes defined in Rust code)

## Basic Usage

```rust
use opaline::{Theme, OpalineColor, OpalineStyle, ThemeVariant};

let theme = Theme::builder("My Theme")
    .author("me")
    .variant(ThemeVariant::Dark)
    .version("1.0")
    .description("A custom dark theme")
    // Palette colors
    .palette("purple", OpalineColor::new(225, 53, 255))
    .palette("cyan", OpalineColor::new(128, 255, 234))
    .palette("dark_bg", OpalineColor::new(18, 18, 24))
    // Semantic tokens
    .token("accent.primary", OpalineColor::new(225, 53, 255))
    .token("accent.secondary", OpalineColor::new(128, 255, 234))
    .token("bg.base", OpalineColor::new(18, 18, 24))
    // Composed styles
    .style("keyword", OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold())
    .style("selected", OpalineStyle::new()
        .with_fg(OpalineColor::new(128, 255, 234))
        .with_bg(OpalineColor::new(30, 30, 40)))
    .build();

assert_eq!(theme.meta.name, "My Theme");
assert!(theme.is_dark());
```

## With Gradients

```rust
use opaline::{Theme, OpalineColor, Gradient};

let theme = Theme::builder("Gradient Theme")
    .token("accent.primary", OpalineColor::new(225, 53, 255))
    .token("accent.secondary", OpalineColor::new(128, 255, 234))
    .gradient("primary", Gradient::new(vec![
        OpalineColor::new(225, 53, 255),
        OpalineColor::new(128, 255, 234),
    ]))
    .gradient("aurora", Gradient::new(vec![
        OpalineColor::new(225, 53, 255),
        OpalineColor::new(128, 255, 234),
        OpalineColor::new(80, 250, 123),
        OpalineColor::new(255, 106, 193),
    ]))
    .build();

assert!(theme.has_gradient("primary"));
assert!(theme.has_gradient("aurora"));
```

## Builder API

All builder methods return `Self` for chaining:

| Method | Description |
|--------|-------------|
| `Theme::builder(name)` | Start building with a theme name |
| `.author(author)` | Set the theme author |
| `.variant(variant)` | Set dark/light variant |
| `.version(version)` | Set theme version string |
| `.description(desc)` | Set theme description |
| `.palette(name, color)` | Add a palette color |
| `.token(name, color)` | Add a semantic token |
| `.style(name, style)` | Add a composed style |
| `.gradient(name, gradient)` | Add a gradient (requires `gradients` feature) |
| `.build()` | Consume the builder and produce a `Theme` |

## Testing Pattern

ThemeBuilder is particularly useful for creating minimal test themes:

```rust
#[cfg(test)]
mod tests {
    use opaline::*;

    fn test_theme() -> Theme {
        Theme::builder("Test")
            .token("accent.primary", OpalineColor::new(255, 0, 0))
            .token("bg.base", OpalineColor::new(0, 0, 0))
            .style("keyword", OpalineStyle::fg(OpalineColor::new(255, 0, 0)).bold())
            .build()
    }

    #[test]
    fn keyword_is_bold_red() {
        let theme = test_theme();
        let kw = theme.style("keyword");
        assert_eq!(kw.fg, Some(OpalineColor::new(255, 0, 0)));
        assert!(kw.bold);
    }
}
```
