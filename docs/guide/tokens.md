# Tokens

Tokens are the **semantic layer** between raw palette colors and composed styles. They give meaning to colors: `accent.primary` means "the main accent color" regardless of whether it's purple, blue, or green.

## Token Namespaces

Opaline's core token contract defines 26 tokens across 6 namespaces:

### Text

| Token            | Purpose             |
| ---------------- | ------------------- |
| `text.primary`   | Main body text      |
| `text.secondary` | Less prominent text |
| `text.muted`     | De-emphasized text  |
| `text.dim`       | Very subtle text    |

### Background

| Token          | Purpose                     |
| -------------- | --------------------------- |
| `bg.base`      | Main background             |
| `bg.panel`     | Panel/sidebar background    |
| `bg.code`      | Code block background       |
| `bg.highlight` | Highlighted line background |
| `bg.selection` | Selection background        |

### Accent

| Token              | Purpose                                |
| ------------------ | -------------------------------------- |
| `accent.primary`   | Main accent (keywords, active states)  |
| `accent.secondary` | Secondary accent (functions, links)    |
| `accent.tertiary`  | Third accent (types, special elements) |
| `accent.deep`      | Deep/saturated accent for emphasis     |

### Status

| Token     | Purpose              |
| --------- | -------------------- |
| `success` | Success states       |
| `error`   | Error states         |
| `warning` | Warning states       |
| `info`    | Informational states |

### Border

| Token              | Purpose                |
| ------------------ | ---------------------- |
| `border.focused`   | Focused panel border   |
| `border.unfocused` | Unfocused panel border |

### Code

| Token              | Purpose           |
| ------------------ | ----------------- |
| `code.keyword`     | Language keywords |
| `code.function`    | Function names    |
| `code.string`      | String literals   |
| `code.number`      | Numeric literals  |
| `code.comment`     | Comments          |
| `code.type`        | Type names        |
| `code.line_number` | Line numbers      |

## Accessing Tokens

```rust
let theme = opaline::Theme::default();

// Get a resolved color (falls back to magenta if missing)
let color = theme.color("accent.primary");

// Strict lookup: None if missing
let color = theme.try_color("accent.primary");

// Check existence
if theme.has_token("accent.primary") {
    // ...
}

// List all token names
let names = theme.token_names();
```

### Name Constants

Use the `names` module for autocomplete-friendly constants instead of raw strings:

```rust
use opaline::names::{tokens, styles, gradients};

let accent = theme.color(tokens::ACCENT_PRIMARY);
let bg = theme.color(tokens::BG_BASE);
let kw = theme.style(styles::KEYWORD);
let has_aurora = theme.has_gradient(gradients::AURORA);
```

All 26 required tokens, 13 required styles, and 5 required gradients have corresponding constants.

## App-Specific Tokens

Git status colors, diff colors, mode indicators, file path/hash colors, and similar domain-specific semantics are intentionally **not** part of Opaline's core contract. Consumer apps should derive those from the generic token set with `register_default_token()` / `register_default_style()`.

## Token Resolution

Tokens resolve through the palette. A token value is a palette key:

```toml
[palette]
electric_purple = "#e135ff"

[tokens]
"accent.primary" = "electric_purple"  # → OpalineColor(225, 53, 255)
```

Tokens can also reference other tokens (transitive resolution):

```toml
[tokens]
"accent.primary" = "electric_purple"
"code.keyword" = "accent.primary"     # → also resolves to #e135ff
```

The resolver detects circular references and reports them as errors.
