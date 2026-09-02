# CSS Adapter

The `css` feature generates CSS custom properties and classes from Opaline themes, bridging your themes into web frameworks like Leptos, Yew, Dioxus, and Tauri.

```toml
[dependencies]
opaline = { version = "0.4", features = ["css"] }
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
  --opaline-accent-primary: #e135ff;
  --opaline-bg-base: #121218;
  --opaline-text-primary: #f8f8f2;
  /* ... 28 token variables */
  --opaline-gradient-primary: linear-gradient(to right, #e135ff, #80ffea);
  --opaline-gradient-aurora: linear-gradient(
    to right,
    #e135ff,
    #f31bff,
    #ff00ff,
    #bf80f4,
    #80ffea
  );
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
  color: #e135ff;
  font-weight: bold;
}

.opaline-error-style {
  color: #ff6363;
}

.opaline-inline-code {
  color: #50fa7b;
  background-color: #1e1e28;
}
```

Style modifiers map to CSS properties:

| OpalineStyle  | CSS Property                    |
| ------------- | ------------------------------- |
| `fg`          | `color`                         |
| `bg`          | `background-color`              |
| `bold`        | `font-weight: bold`             |
| `dim`         | `opacity: 0.7`                  |
| `italic`      | `font-style: italic`            |
| `underline`   | `text-decoration: underline`    |
| `crossed_out` | `text-decoration: line-through` |
| `hidden`      | `visibility: hidden`            |

`reversed`, `slow_blink`, and `rapid_blink` have no CSS equivalent and are skipped.

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
