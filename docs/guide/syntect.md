# Syntect Adapter

The `syntect` feature bridges Opaline themes into the [syntect](https://crates.io/crates/syntect) syntax highlighting ecosystem — used by bat, delta, and many other tools.

```toml
[dependencies]
opaline = { version = "0.2", features = ["syntect"] }
```

## Color Conversion

`OpalineColor` converts to `syntect::highlighting::Color`:

```rust
use opaline::OpalineColor;
use syntect::highlighting::Color;

let color = OpalineColor::new(225, 53, 255);
let st_color: Color = color.into();
// → Color { r: 225, g: 53, b: 255, a: 255 }
```

## Style Conversion

`OpalineStyle` converts to `syntect::highlighting::StyleModifier`:

```rust
use opaline::OpalineStyle;
use syntect::highlighting::StyleModifier;

let style = OpalineStyle::fg(OpalineColor::new(203, 166, 247)).bold().italic();
let modifier: StyleModifier = style.into();
// foreground set, BOLD | ITALIC font_style
```

Modifier mapping:

| OpalineStyle | FontStyle |
|-------------|-----------|
| `bold` | `BOLD` |
| `italic` | `ITALIC` |
| `underline` | `UNDERLINE` |

## Theme Generation

Convert a full Opaline theme to a syntect `Theme`:

```rust
use opaline::adapters::syntect::to_syntect_theme;

let theme = opaline::Theme::default();
let syntect_theme = to_syntect_theme(&theme);

// Use with syntect's highlighter
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;

let ps = SyntaxSet::load_defaults_newlines();
let syntax = ps.find_syntax_by_extension("rs").unwrap();
let mut h = HighlightLines::new(syntax, &syntect_theme);
```

### Token → Scope Mapping

The adapter maps Opaline's generic core tokens to standard TextMate scopes:

| Opaline Token | TextMate Scope |
|--------------|----------------|
| `code.keyword` | `keyword` |
| `code.string` | `string` |
| `code.comment` | `comment` |
| `code.function` | `entity.name.function, support.function` |
| `code.number` | `constant.numeric` |
| `code.type` | `entity.name.type, support.type` |
| `code.line_number` | `constant.numeric.line-number` |
| `accent.primary` | `variable` |
| `accent.secondary` | `storage.type, storage.modifier` |
| `accent.tertiary` | `constant.other, variable.other.constant` |
| `success` | `markup.inserted` |
| `warning` | `markup.changed` |
| `error` | `invalid, message.error` |
| `info` | `string.other.link, support.constant` |

### ThemeSettings Mapping

Editor-level settings are derived from semantic tokens:

| Opaline Token | ThemeSettings Field |
|--------------|-------------------|
| `text.primary` | `foreground` |
| `bg.base` | `background` |
| `accent.primary` | `caret`, `accent` |
| `bg.highlight` | `line_highlight` |
| `bg.selection` | `selection` |
| `bg.panel` | `gutter` |
| `text.dim` | `gutter_foreground` |
| `border.focused` | `active_guide` |
| `border.unfocused` | `guide` |

### Style Modifiers

If a named style exists that matches a `code.*` token (for example, `keyword` for `code.keyword`), its modifiers are included in the generated scope.
