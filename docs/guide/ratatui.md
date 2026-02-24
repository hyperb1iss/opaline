# Ratatui Adapter

The `ratatui` feature (enabled by default) provides seamless integration between Opaline themes and Ratatui's rendering types.

## From Impls

Opaline types convert directly to Ratatui types:

```rust
use opaline::OpalineColor;
use ratatui::style::Color;

let opaline_color = OpalineColor::new(225, 53, 255);
let ratatui_color: Color = opaline_color.into();
// → Color::Rgb(225, 53, 255)
```

Works by value and by reference:

```rust
let color = OpalineColor::new(128, 255, 234);
let rat: Color = Color::from(&color); // from reference
let rat: Color = color.into();        // from value
```

## ThemeRatatuiExt

The `ThemeRatatuiExt` trait adds Ratatui-specific methods to `Theme`:

```rust
use opaline::{Theme, ThemeRatatuiExt};

let theme = Theme::default();

// Get a ratatui Color from a token
let color: ratatui::style::Color = theme.ratatui_color("accent.primary");

// Get a ratatui Style from a named style
let style: ratatui::style::Style = theme.ratatui_style("keyword");

// Create a styled Span
let span: ratatui::text::Span = theme.ratatui_span("keyword", "fn");

// Create a styled Line
let line: ratatui::text::Line = theme.ratatui_line("keyword", "let x = 42;");

// Create a styled Text block
let text: ratatui::text::Text = theme.ratatui_text("keyword", "multi\nline\ntext");
```

## Styled Trait

`OpalineStyle` implements Ratatui's `Styled` trait:

```rust
use opaline::OpalineStyle;
use ratatui::style::Stylize;
use ratatui::text::Span;

let style = OpalineStyle::fg(OpalineColor::new(225, 53, 255)).bold();
let rat_style: ratatui::style::Style = style.style();
```

## Modifier Mapping

All 9 Opaline modifiers map to their Ratatui equivalents:

| Opaline | Ratatui |
|---------|---------|
| `bold` | `Modifier::BOLD` |
| `italic` | `Modifier::ITALIC` |
| `underline` | `Modifier::UNDERLINED` |
| `dim` | `Modifier::DIM` |
| `strikethrough` | `Modifier::CROSSED_OUT` |
| `reversed` | `Modifier::REVERSED` |
| `hidden` | `Modifier::HIDDEN` |
| `rapid_blink` | `Modifier::RAPID_BLINK` |
| `slow_blink` | `Modifier::SLOW_BLINK` |

## Gradient Helpers

With both `ratatui` and `gradients` features enabled, you get gradient rendering functions:

```rust
use opaline::{Theme, gradient_spans, gradient_line, gradient_bar, gradient_text_line};

let theme = Theme::default();

// Vec<Span> — each character colored along the gradient
let spans: Vec<ratatui::text::Span> = gradient_spans(&theme, "aurora", "Rainbow text!");

// Line — ready to render
let line: ratatui::text::Line = gradient_line(&theme, "primary", "Status bar");

// Progress bar using block characters
let bar: ratatui::text::Line = gradient_bar(&theme, "warm", 30);

// Text with gradient applied
let styled: ratatui::text::Line = gradient_text_line(&theme, "primary", "Loading...");
```

## In Your Render Function

Typical usage inside a Ratatui `render` callback:

```rust
use opaline::{Theme, ThemeRatatuiExt, gradient_bar};
use ratatui::widgets::{Block, Borders, Paragraph};

fn render(frame: &mut ratatui::Frame, theme: &Theme) {
    let area = frame.area();

    // Themed block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.ratatui_style("focused_border"))
        .title_style(theme.ratatui_style("keyword"));

    // Themed paragraph
    let text = theme.ratatui_text("keyword", "Hello, Opaline!");
    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, area);
}
```
