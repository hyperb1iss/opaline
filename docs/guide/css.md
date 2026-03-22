# CSS Adapter

The `css` feature generates CSS custom properties and classes from Opaline themes — bridging your themes into web frameworks like Leptos, Yew, Dioxus, and Tauri.

```toml
[dependencies]
opaline = { version = "0.2", features = ["css"] }
```

## CSS Custom Properties

Generate `:root` variables from all theme tokens:

```rust
use opaline::{Theme, generate_css_vars};

let theme = Theme::default();
let css = generate_css_vars(&theme);
```

Output:

```css
:root {
  --opaline-accent-primary: #cba6f7;
  --opaline-bg-base: #1e1e2e;
  --opaline-text-primary: #cdd6f4;
  /* ... 39 token variables */
  --opaline-gradient-primary: linear-gradient(to right, #e135ff, #80ffea);
  --opaline-gradient-aurora: linear-gradient(to right, #e135ff, #80ffea, #ff6ac1);
}
```

Token names are prefixed with `--opaline-` and dots/underscores become dashes.

## CSS Classes

Generate classes from theme styles:

```rust
use opaline::{Theme, generate_css_classes};

let theme = Theme::default();
let css = generate_css_classes(&theme);
```

Output:

```css
.opaline-keyword {
  color: #cba6f7;
  font-weight: bold;
}

.opaline-error-style {
  color: #f38ba8;
}

.opaline-diff-added {
  color: #a6e3a1;
}
```

Style modifiers map to CSS properties:

| OpalineStyle | CSS Property |
|-------------|-------------|
| `fg` | `color` |
| `bg` | `background-color` |
| `bold` | `font-weight: bold` |
| `dim` | `opacity: 0.7` |
| `italic` | `font-style: italic` |
| `underline` | `text-decoration: underline` |
| `crossed_out` | `text-decoration: line-through` |
| `hidden` | `visibility: hidden` |

## Complete Stylesheet

Generate both variables and classes in one call:

```rust
use opaline::{Theme, generate_stylesheet};

let theme = Theme::default();
let css = generate_stylesheet(&theme);
// Includes :root { ... } + .opaline-* classes
```

## Usage with Web Frameworks

### Leptos / Yew / Dioxus

Generate CSS at build time and include in your app:

```rust
// build.rs
let theme = opaline::Theme::default();
let css = opaline::generate_stylesheet(&theme);
std::fs::write("style/theme.css", css).unwrap();
```

### Tauri

Generate CSS and inject via the webview:

```rust
let css = opaline::generate_stylesheet(&theme);
window.eval(&format!("
    const style = document.createElement('style');
    style.textContent = `{css}`;
    document.head.appendChild(style);
"))?;
```

### Runtime Theme Switching

Regenerate CSS when the theme changes:

```rust
fn switch_theme(name: &str) -> String {
    let theme = opaline::load_by_name(name).expect("valid theme");
    opaline::generate_stylesheet(&theme)
}
```
