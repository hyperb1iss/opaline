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

## Theme Methods

`Theme` has inherent methods for creating Ratatui text types — no trait import needed:

```rust
use opaline::Theme;

let theme = Theme::default();

// Create styled text elements directly
let span: ratatui::text::Span = theme.span("keyword", "fn");
let line: ratatui::text::Line = theme.line("keyword", "let x = 42;");
let text: ratatui::text::Text = theme.text("keyword", "multi\nline\ntext");

// Gradient-colored text (requires `gradients` feature)
let grad: ratatui::text::Line = theme.gradient_text("aurora", "✦ Opaline");
```

## Style Integration

`OpalineStyle` implements `Into<Style>`, so `theme.style()` works directly with any Ratatui widget method that accepts `impl Into<Style>`:

```rust
use ratatui::widgets::Block;

// Works directly — no conversion needed
Block::bordered()
    .style(theme.style("keyword"))
    .border_style(theme.style("focused_border"));

// For Style::fg() / Style::bg(), use .into() on colors
use ratatui::style::Style;
let bg = Style::default().bg(theme.color("bg.base").into());
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

With both `ratatui` and `gradients` features enabled, you get gradient rendering helpers.

### High-Level (Theme Methods)

```rust
// Per-character gradient coloring → Line
let line = theme.gradient_text("aurora", "Rainbow text!");
```

### Low-Level (Free Functions)

These take a `&Gradient` directly for maximum control:

```rust
use opaline::{gradient_spans, gradient_text_line, gradient_bar};

let gradient = theme.get_gradient("primary").unwrap();

// Vec<Span> — each character colored along the gradient
let spans = gradient_spans("Hello!", gradient);

// Line — gradient-colored text
let line = gradient_text_line("Status: Online", gradient);

// Line — repeated block characters for progress bars
let bar = gradient_bar(40, '█', gradient);
```

## In Your Render Function

Typical usage inside a Ratatui `render` callback:

```rust
use opaline::{Theme, gradient_bar};
use ratatui::widgets::{Block, Borders, Paragraph};

fn render(frame: &mut ratatui::Frame, theme: &Theme) {
    let area = frame.area();

    // Themed block — OpalineStyle works via Into<Style>
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.style("focused_border"))
        .title_style(theme.style("keyword"));

    // Themed paragraph
    let text = theme.text("keyword", "Hello, Opaline!");
    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, area);
}
```
