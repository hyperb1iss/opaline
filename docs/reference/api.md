# API Reference

This is an overview of Opaline's public API. For full docs.rs documentation, run `cargo doc --all-features --open`.

## Core Types

### `OpalineColor`

An RGB color value with hex parsing, interpolation, and format conversions.

```rust
use opaline::OpalineColor;

// Construction
let color = OpalineColor::new(225, 53, 255);
let color = OpalineColor::from_hex("#e135ff")?;
let color = OpalineColor::from((225, 53, 255));
let color = OpalineColor::from([225, 53, 255]);

// Access
let (r, g, b) = (color.r, color.g, color.b);

// Interpolation
let mid = OpalineColor::lerp(&start, &end, 0.5);

// Display
assert_eq!(format!("{color}"), "#e135ff");

// Constants
OpalineColor::BLACK              // pure black
OpalineColor::WHITE              // pure white
OpalineColor::FALLBACK           // neutral gray (128, 128, 128)

// Color manipulation
color.darken(0.3)                // mix 30% toward black
color.lighten(0.2)               // mix 20% toward white
color.desaturate(0.5)            // mix 50% toward luminance gray
```

### `OpalineStyle`

A composed style with foreground, background, and 9 text modifiers.

```rust
use opaline::OpalineStyle;

// Builder pattern
let style = OpalineStyle::fg(color)
    .with_bg(bg_color)
    .bold()
    .italic()
    .underline()
    .dim()
    .strikethrough()
    .reversed()
    .hidden()
    .rapid_blink()
    .slow_blink();

// Static constructors
let fg_only = OpalineStyle::fg(color);
let empty = OpalineStyle::new();

// Merge (overlay non-default fields)
let merged = base.merge(&overlay);
```

### `Theme`

A fully resolved theme with accessors for colors, styles, and gradients.

```rust
use opaline::Theme;

// Construction
let theme = Theme::default();
let theme = Theme::builder("name").build();
let theme = opaline::load_from_str(toml)?;
let theme = opaline::load_from_file("path.toml")?;
let theme = opaline::load_by_name("dracula")?;

// Color access
theme.color("token.name")           // OpalineColor (fallback on miss)
theme.try_color("token.name")       // Option<OpalineColor>
theme.has_token("token.name")       // bool
theme.token_names()                 // Vec<&str>
theme.palette_names()               // Vec<&str>

// Style access
theme.style("name")                 // OpalineStyle (default on miss)
theme.try_style("name")             // Option<&OpalineStyle>
theme.has_style("name")             // bool
theme.style_names()                 // Vec<&str>

// Gradient access (requires `gradients` feature)
theme.gradient("name", 0.5)         // OpalineColor (fallback on miss)
theme.try_gradient("name", 0.5)     // Option<OpalineColor>
theme.get_gradient("name")          // Option<&Gradient>
theme.has_gradient("name")          // bool
theme.gradient_names()              // Vec<&str>

// Metadata
theme.meta.name                     // String
theme.meta.author                   // Option<String>
theme.meta.variant                  // ThemeVariant
theme.is_dark()                     // bool
theme.is_light()                    // bool

// Token injection (app-level derivation)
theme.register_default_token("name", color)   // insert if absent (TOML wins)
theme.register_token("name", color)           // unconditional overwrite
theme.register_default_style("name", style)   // insert if absent
theme.register_style("name", style)           // unconditional overwrite
```

### `ThemeBuilder`

Programmatic theme construction.

```rust
Theme::builder("name")
    .author("author")
    .variant(ThemeVariant::Dark)
    .version("1.0")
    .description("desc")
    .palette("name", color)
    .token("name", color)
    .style("name", style)
    .gradient("name", gradient)       // requires `gradients`
    .build()                          // -> Theme
```

### `Gradient`

Multi-stop color interpolation.

```rust
use opaline::Gradient;

let grad = Gradient::new(vec![color_a, color_b, color_c]);
let grad = Gradient::try_new(vec![color_a])?;

grad.at(0.5)           // OpalineColor at midpoint
grad.generate(10)      // Vec<OpalineColor> evenly spaced
grad.len()             // usize
grad.stops()           // &[OpalineColor]
```

## Names Module

Autocomplete-friendly constants for the standard theme contract. Use these instead of raw strings for typo prevention.

```rust
use opaline::names::{tokens, styles, gradients};

theme.color(tokens::ACCENT_PRIMARY)     // OpalineColor
theme.style(styles::KEYWORD)            // OpalineStyle
theme.has_gradient(gradients::AURORA)    // bool
```

Modules: `names::tokens` (38 constants), `names::styles` (18 constants), `names::gradients` (5 constants).

## Ratatui Integration

Requires `ratatui` feature (default). No trait import needed — methods are inherent on `Theme`.

### Theme Methods

```rust
// Colors/styles use Into conversions
let color: ratatui::style::Color = theme.color("token").into();
let style: ratatui::style::Style = theme.style("name").into();

// Or pass theme.style() directly to widget methods (via Into<Style>)
Block::bordered().style(theme.style("keyword"));

// Text type builders
theme.span("name", "text")             // ratatui::text::Span
theme.line("name", "text")             // ratatui::text::Line
theme.text("name", "text")             // ratatui::text::Text
theme.gradient_text("name", "text")    // ratatui::text::Line (gradients feature)
```

### Gradient Helpers

Requires `ratatui` + `gradients` features. Low-level free functions take a `&Gradient` directly:

```rust
use opaline::{gradient_spans, gradient_text_line, gradient_bar, gradient_line};

let grad = theme.get_gradient("name").unwrap();

gradient_spans("text", &grad)               // Vec<Span>
gradient_text_line("text", &grad)            // Line
gradient_bar(width, '█', &grad)              // Line (block chars)
gradient_line(width, '─', &grad)             // Vec<Span>
```

## CLI Integration

Requires `cli` feature.

### `ThemeCliExt` Trait

```rust
use opaline::ThemeCliExt;

theme.colorize("token", "text")     // ColoredString
theme.style_text("name", "text")    // ColoredString
```

### `ColoredExt` Trait

```rust
use opaline::ColoredExt;

"text".opaline_fg(color)            // ColoredString
```

### Gradient String

Requires `cli` + `gradients` features.

```rust
use opaline::gradient_string;

gradient_string(&theme, "name", "text")  // ColoredString
```

## Global State

Requires `global-state` feature.

```rust
use opaline::{current, set_theme, load_theme, load_theme_by_name};

let theme = current();                          // Arc<Theme>
set_theme(theme);                               // replace global theme
load_theme(path)?;                              // load from file + set
load_theme_by_name("dracula")?;                 // load builtin + set

// With app-level derivation callback
use opaline::load_theme_by_name_with;
load_theme_by_name_with("dracula", |theme| {    // load + derive + set
    theme.register_default_token("nav.bg", theme.color("bg.base").darken(0.1));
})?;

// App-specific discovery paths (requires `discovery` too)
use opaline::{load_theme_by_name_for_app, load_theme_by_name_for_app_with};
load_theme_by_name_for_app("custom", "myapp")?;
load_theme_by_name_for_app_with("custom", "myapp", derive_fn)?;
```

## Builtins

Requires `builtin-themes` feature (default).

```rust
use opaline::{load_by_name, list_available_themes, ThemeInfo};

let theme = load_by_name("nord")?;              // Option<Theme>
let themes: Vec<ThemeInfo> = list_available_themes();
```

## Widgets

Requires `widgets` feature.

```rust
use opaline::{ThemeSelector, ThemeSelectorState, ThemeSelectorAction};

// Create state
let state = ThemeSelectorState::new();
let state = ThemeSelectorState::with_current_selected();
let state = ThemeSelectorState::with_current_selected()
    .with_derive(derive_fn);                   // app derivation for preview

// Handle input
let action = state.handle_key(key_event);      // ThemeSelectorAction

// Query state
state.selected_theme()                         // Option<&ThemeInfo>
state.filter()                                 // &str

// Render
let widget = ThemeSelector::new();
let widget = ThemeSelector::new().title("Pick Theme");
frame.render_stateful_widget(widget, area, &mut state);
```

`ThemeSelectorAction` variants: `Navigate`, `Select(String)`, `Cancel`, `FilterChanged`, `Noop`.

## Discovery

Requires `discovery` feature.

```rust
use opaline::{app_theme_dirs, theme_dirs};

let dirs = app_theme_dirs("myapp");             // Vec<PathBuf>
let dirs = theme_dirs();                        // Vec<PathBuf>
```
