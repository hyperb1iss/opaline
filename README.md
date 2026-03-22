<h1 align="center">
  <br>
  ✦ opaline
  <br>
</h1>

<p align="center">
  <strong>A token-based theme engine for Rust applications</strong><br>
  <sub>39 builtin themes &middot; semantic tokens &middot; multi-stop gradients &middot; ratatui &middot; egui &middot; crossterm &middot; syntect &middot; CSS</sub>
</p>

<p align="center">
  <a href="https://crates.io/crates/opaline">
    <img src="https://img.shields.io/crates/v/opaline.svg?style=for-the-badge&logo=rust&logoColor=white&color=e135ff" alt="Crates.io">
  </a>
  <a href="https://docs.rs/opaline">
    <img src="https://img.shields.io/docsrs/opaline?style=for-the-badge&logo=docs.rs&logoColor=white&color=80ffea" alt="docs.rs">
  </a>
  <a href="https://github.com/hyperb1iss/opaline/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/hyperb1iss/opaline/ci.yml?style=for-the-badge&logo=github-actions&logoColor=white&color=4C566A" alt="CI">
  </a>
  <a href="#-builtin-themes">
    <img src="https://img.shields.io/badge/Themes-39_Built--in-ff6ac1?style=for-the-badge&logo=palette&logoColor=white" alt="39 Themes">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-50fa7b?style=for-the-badge&logoColor=white" alt="License">
  </a>
</p>

<p align="center">
  <a href="https://hyperb1iss.github.io/opaline/">Docs</a> &bull;
  <a href="https://hyperb1iss.github.io/opaline/llms.txt">llms.txt</a> &bull;
  <a href="#-quick-start">Quick Start</a> &bull;
  <a href="#-features">Features</a> &bull;
  <a href="#-builtin-themes">Themes</a> &bull;
  <a href="#-usage">Usage</a> &bull;
  <a href="#-custom-themes">Custom Themes</a> &bull;
  <a href="#-contributing">Contributing</a>
</p>

---

<div align="center">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-silkcircuit-neon.png" alt="SilkCircuit Neon" width="49%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-silkcircuit-dawn.png" alt="SilkCircuit Dawn" width="49%">
</div>

<p align="center"><em>SilkCircuit Neon (dark) and SilkCircuit Dawn (light) — two of 39 builtin themes</em></p>

---

## 💜 What is Opaline?

Opaline is a **theme engine** that brings consistent, beautiful color to Rust applications. Instead of scattering hex codes across your codebase, you define themes as TOML files with a **palette → token → style → gradient** resolution pipeline. Switch themes at runtime with a single call — every widget updates instantly.

Opaline ships adapters for **ratatui**, **egui**, **crossterm**, **owo-colors**, **syntect**, and **CSS** — one theme definition, every rendering target.

```
TOML file → ThemeFile (serde) → Resolver (palette → tokens → styles → gradients) → Theme
```

Opaline ships with **39 professionally crafted themes** spanning 17 colorscheme families, all enforced by a strict contract test suite that validates 26 core semantic tokens, 13 required styles, and 5 gradients per theme.

## ✦ Features

| Feature | Description |
| --- | --- |
| 🎨 **39 Builtin Themes** | SilkCircuit, Catppuccin, GitHub, Monokai Pro, Ayu, Night Owl, Flexoki, Palenight, Dracula, Nord, Rose Pine, Gruvbox, Solarized, Tokyo Night, Kanagawa, Everforest, One Dark/Light |
| 🔗 **Semantic Tokens** | 26 core tokens across generic `text.*`, `bg.*`, `accent.*`, `border.*`, and `code.*` namespaces |
| 🌊 **Multi-Stop Gradients** | Smooth color interpolation with `gradient_bar()`, `gradient_text_line()`, and `gradient_spans()` |
| 🖥️ **Deep Ratatui Integration** | `From` impls, `Styled` trait, inherent `span()`, `line()`, `text()`, `gradient_text()` on `Theme` |
| 🎮 **egui Integration** | `Color32` conversion, full `Visuals` generation from theme tokens |
| ⌨️ **Crossterm Adapter** | Direct `Color`/`ContentStyle` conversion with gradient rendering |
| 🌈 **owo-colors Adapter** | Zero-allocation terminal coloring with `Style` conversion |
| 🖌️ **Syntax Highlighting** | Generate [syntect](https://crates.io/crates/syntect) themes — powers bat, delta, and more |
| 🌐 **CSS Generation** | Custom properties + classes for web frameworks (Leptos, Yew, Dioxus, Tauri) |
| 🎛️ **ThemeSelector Widget** | Drop-in theme picker with live preview, search filtering, and cancel/restore |
| 🔬 **Color Manipulation** | `darken()`, `lighten()`, `desaturate()` for deriving colors from theme palettes |
| 🏗️ **ThemeBuilder** | Programmatic theme construction without TOML — perfect for runtime customization |
| 🧩 **App-Level Derivation** | Register app-specific tokens/styles with `register_default_token()` — TOML overrides respected |
| 🔍 **Theme Discovery** | Scan `~/.config/` for user themes, list metadata for picker UIs |
| 🌐 **Global State** | Optional process-wide `current()`/`set_theme()` behind a feature flag |
| 🛡️ **Strict Resolution** | Cycle detection, unresolvable token errors, compile-time theme validation |
| 🖨️ **CLI Adapter** | `colored` crate integration for ANSI terminal output |
| ⚡ **Zero Cost Builtins** | Themes embedded via `include_str!` at compile time — no file I/O at runtime |

## ⚡ Quick Start

Add opaline to your `Cargo.toml`:

```toml
[dependencies]
opaline = "0.2"
```

Load a theme and start styling:

```rust
use opaline::load_by_name;

// Load any builtin theme
let theme = load_by_name("catppuccin-mocha").expect("theme exists");

// Use semantic colors and styles in your Ratatui widgets
let style = theme.style("keyword");               // bold accent color
let color = theme.color("accent.primary");         // OpalineColor
let span = theme.span("muted", "src/main.rs");     // styled Span
```

### Run the interactive demo

```bash
cargo run --example theme-showcase
```

Browse all 39 themes, see every style and gradient rendered in real-time.

## 🎨 Builtin Themes

<div align="center">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-catppuccin-mocha.png" alt="Catppuccin Mocha" width="49%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-rose-pine.png" alt="Rose Pine" width="49%">
</div>

<div align="center">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-catppuccin-latte.png" alt="Catppuccin Latte" width="49%">
  <img src="https://raw.githubusercontent.com/hyperb1iss/opaline/main/docs/images/showcase-everforest-light.png" alt="Everforest Light" width="49%">
</div>

| Family | Variants | Character |
| --- | --- | --- |
| **SilkCircuit** | Neon, Soft, Glow, Vibrant, Dawn | Electric meets elegant — the signature design language |
| **Catppuccin** | Mocha, Macchiato, Frappé, Latte | Soothing pastels across four flavors |
| **GitHub** | Dark Dimmed, Light | Clean, familiar, institutional |
| **Monokai Pro** | Classic | The iconic warm vivid syntax palette |
| **Ayu** | Dark, Mirage, Light | Calm, modern, bright across three modes |
| **Night Owl** | Dark, Light | Accessibility-designed deep blue |
| **Flexoki** | Dark, Light | Ink-on-paper aesthetic, Oklab-designed |
| **Palenight** | — | Soft purple-blue pastel sci-fi |
| **Rose Pine** | Base, Moon, Dawn | Botanical elegance across three variants |
| **Everforest** | Dark, Light | Warm green forest tones |
| **Tokyo Night** | Default, Storm, Moon | Neo-Tokyo neon aesthetic |
| **Kanagawa** | Wave, Dragon, Lotus | The great wave — dark, darker, light |
| **Dracula** | — | The classic dark syntax theme |
| **Nord** | — | Arctic, north-bluish clean |
| **Gruvbox** | Dark, Light | Retro groove with warm contrast |
| **Solarized** | Dark, Light | Precision colors for machines and people |
| **One** | Dark, Light | Atom's iconic syntax palette |

Every theme is contract-tested: 26 core semantic tokens, 13 required styles, 5 required gradients.

## 🔮 Usage

### Colors and Styles

```rust
use opaline::Theme;

let theme = Theme::default(); // SilkCircuit Neon

// Semantic color access
let primary = theme.color("accent.primary");
let bg = theme.color("bg.base");

// Composed styles (fg + bg + modifiers)
let keyword = theme.style("keyword");           // bold accent
let error = theme.style("error_style");         // red foreground
let selected = theme.style("active_selected");  // accent on highlight bg

// Styled spans for inline text — no trait import needed
let path = theme.span("muted", "src/lib.rs");
let keyword_span = theme.span("keyword", "fn");
```

### Gradients

```rust
use opaline::{Theme, gradient_bar};

let theme = Theme::default();

// Render a gradient progress bar
if let Some(gradient) = theme.get_gradient("aurora") {
    let bar = gradient_bar(40, '█', gradient); // Line with per-char colors
}

// Gradient-styled text (each character gets interpolated color)
let title = theme.gradient_text("primary", "Opaline Theme Engine");
```

### Theme Switching

```rust
use opaline::{list_available_themes, load_by_name};

// Enumerate all themes for a picker UI
let themes = list_available_themes();
for info in &themes {
    let author = if info.author.is_empty() { "—" } else { &info.author };
    println!("{} ({:?}) by {}", info.display_name, info.variant, author);
}

// Hot-swap themes at runtime
let dracula = load_by_name("dracula").unwrap();
let nord = load_by_name("nord").unwrap();
```

### ThemeBuilder (Programmatic)

```rust
use opaline::{OpalineColor, OpalineStyle, Theme};

let theme = Theme::builder("My Theme")
    .palette("bg", OpalineColor::new(26, 27, 38))
    .palette("fg", OpalineColor::new(192, 202, 245))
    .palette("blue", OpalineColor::new(122, 162, 247))
    .token("text.primary", OpalineColor::new(192, 202, 245))
    .token("bg.base", OpalineColor::new(26, 27, 38))
    .token("accent.primary", OpalineColor::new(122, 162, 247))
    .style("keyword", OpalineStyle::fg(OpalineColor::new(122, 162, 247)).bold())
    .build();
```

## 🪄 Custom Themes

Add a builtin theme by dropping a `.toml` file in `src/builtins/`. For user themes, load from any path at runtime or place the file in your app's theme directory.

```toml
[meta]
name = "My Theme"
author = "your name"
variant = "dark"   # or "light"
description = "A custom theme"

[palette]
bg = "#1a1b26"
fg = "#c0caf5"
blue = "#7aa2f7"
purple = "#bb9af7"

[tokens]
"text.primary" = "fg"
"bg.base" = "bg"
"bg.selection" = "bg"
"accent.primary" = "blue"
# ... 26 required core tokens across text.*, bg.*, accent.*, border.*, code.*, etc.

[styles]
keyword = { fg = "accent.primary", bold = true }
# ... 13 required core styles

[gradients]
primary = ["blue", "purple"]
# ... 5 required gradients
```

The resolver validates everything at load time — circular references, missing tokens, and invalid colors all produce clear error messages via `OpalineError`.

## ⚙️ Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `builtin-themes` | ✓ | 39 embedded TOML themes via `include_str!` |
| `gradients` | ✓ | Multi-stop gradient interpolation |
| `ratatui` | ✓ | `From` impls, inherent `span()`/`line()`/`text()`/`gradient_text()` |
| `cli` | — | `colored` crate adapter for ANSI output |
| `crossterm` | — | Direct crossterm `Color`/`ContentStyle` adapter |
| `owo-colors` | — | Zero-allocation terminal coloring |
| `css` | — | CSS custom properties + classes generation |
| `syntect` | — | Syntax highlighting theme generation |
| `egui` | — | egui `Visuals`/`Color32` adapter |
| `global-state` | — | Process-wide `current()`/`set_theme()` |
| `discovery` | — | Load user themes from `~/.config/` |
| `widgets` | — | Theme selector widget with live preview |

## 🏗️ Architecture

```
TOML → ThemeFile (serde) → Resolver → Theme
         │                    │          │
         │  palette           │          ├── color("token.name") → OpalineColor
         │  tokens            │          ├── style("style_name") → OpalineStyle
         │  styles            │          ├── gradient("name") → Gradient
         │  gradients         │          └── meta (name, author, variant)
         │                    │
         │                    ├── palette → token resolution
         │                    ├── token → style resolution
         │                    ├── cycle detection
         │                    └── gradient stop resolution
```

| Component | Purpose |
| --- | --- |
| `OpalineColor` | RGB color with hex/tuple/array/u32 conversions + lerp + darken/lighten/desaturate |
| `OpalineStyle` | Composed style (fg, bg, 9 modifiers) with builder pattern |
| `Gradient` | Multi-stop color interpolation with `at(t)` and `generate(n)` |
| `Theme` | Fully resolved theme with `color()`, `style()`, `gradient()` accessors |
| `ThemeBuilder` | Programmatic theme construction without TOML |
| `ThemeInfo` | Metadata for theme discovery and picker UIs |
| `OpalineError` | Typed errors for IO, parsing, resolution, and validation failures |

## 🧪 Development

```bash
cargo check                               # Fast type check
cargo clippy --all-targets --all-features  # Pedantic lint gate
cargo test --all-features                  # Full test suite (203 tests)
cargo doc --all-features --open            # Generate docs
cargo run --example theme-showcase         # Interactive TUI demo
```

Requires **Rust 1.85+** (Edition 2024). `unsafe_code = "forbid"`, `clippy::pedantic` deny.

## 🤝 Contributing

Contributions welcome! Adding a new builtin theme is as easy as dropping a `.toml` file in `src/builtins/` — it's auto-discovered at compile time. Run `cargo test --all-features` to validate against the contract test suite.

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for details.

---

<div align="center">

📖 [Documentation](https://hyperb1iss.github.io/opaline/) · 📦 [API Reference](https://docs.rs/opaline) · 🐛 [Report Bug](https://github.com/hyperb1iss/opaline/issues) · 💡 [Request Feature](https://github.com/hyperb1iss/opaline/issues)

</div>

<div align="center">

Created by [Stefanie Jane 🌠](https://github.com/hyperb1iss)

If you find this useful, [buy me a Monster Ultra Violet](https://ko-fi.com/hyperb1iss)! ⚡️

</div>
