<h1 align="center">
  <br>
  ✦ opaline
  <br>
</h1>

<p align="center">
  <strong>A token-based theme engine for Ratatui TUI applications</strong><br>
  <sub>20 builtin themes &middot; semantic tokens &middot; multi-stop gradients &middot; zero unsafe</sub>
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
    <img src="https://img.shields.io/badge/Themes-20_Built--in-ff6ac1?style=for-the-badge&logo=palette&logoColor=white" alt="20 Themes">
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

<p align="center"><em>SilkCircuit Neon (dark) and SilkCircuit Dawn (light) — two of 20 builtin themes</em></p>

---

## 💜 What is Opaline?

Opaline is a **theme engine** that brings consistent, beautiful color to [Ratatui](https://ratatui.rs) terminal applications. Instead of scattering hex codes across your codebase, you define themes as TOML files with a **palette → token → style → gradient** resolution pipeline. Switch themes at runtime with a single call — every widget updates instantly.

```
TOML file → ThemeFile (serde) → Resolver (palette → tokens → styles → gradients) → Theme
```

Opaline ships with **20 professionally crafted themes** spanning 8 colorscheme families, all enforced by a strict contract test suite that validates 40+ semantic tokens, 18 styles, and 5 gradients per theme.

## ✦ Features

| Feature | Description |
| --- | --- |
| 🎨 **20 Builtin Themes** | SilkCircuit, Catppuccin, Dracula, Nord, Rose Pine, Gruvbox, Solarized, Tokyo Night, Kanagawa, Everforest, One Dark |
| 🔗 **Semantic Tokens** | 40+ tokens across `text.*`, `bg.*`, `accent.*`, `git.*`, `diff.*`, `code.*` namespaces |
| 🌊 **Multi-Stop Gradients** | Smooth color interpolation with `gradient_bar()`, `gradient_text_line()`, and `gradient_spans()` |
| 🖥️ **Deep Ratatui Integration** | `From` impls, `Styled` trait, inherent `span()`, `line()`, `text()`, `gradient_text()` on `Theme` |
| 🎛️ **ThemeSelector Widget** | Drop-in theme picker with live preview, search filtering, and cancel/restore |
| 🔬 **Color Manipulation** | `darken()`, `lighten()`, `desaturate()` for deriving colors from theme palettes |
| 🏗️ **ThemeBuilder** | Programmatic theme construction without TOML — perfect for runtime customization |
| 🧩 **App-Level Derivation** | Register app-specific tokens/styles with `register_default_token()` — TOML overrides respected |
| 🔍 **Theme Discovery** | Scan `~/.config/` for user themes, list metadata for picker UIs |
| 🌐 **Global State** | Optional process-wide `current()`/`set_theme()` behind a feature flag |
| 🛡️ **Strict Resolution** | Cycle detection, unresolvable token errors, compile-time theme validation |
| 🖨️ **CLI Adapter** | `colored` crate integration for ANSI terminal output outside of Ratatui |
| ⚡ **Zero Cost Builtins** | Themes embedded via `include_str!` at compile time — no file I/O at runtime |

## ⚡ Quick Start

Add opaline to your `Cargo.toml`:

```toml
[dependencies]
opaline = "0.1"
```

Load a theme and start styling:

```rust
use opaline::load_by_name;

// Load any of 20 builtin themes
let theme = load_by_name("catppuccin-mocha").expect("theme exists");

// Use semantic colors and styles in your Ratatui widgets
let style = theme.style("keyword");               // bold accent color
let color = theme.color("accent.primary");         // OpalineColor
let span = theme.span("file_path", "src/main.rs"); // styled Span
```

### Run the interactive demo

```bash
cargo run --example theme-showcase
```

Browse all 20 themes, see every style and gradient rendered in real-time.

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
| **Catppuccin** | Mocha, Latte | Soothing pastels for dark and light |
| **Rose Pine** | Base, Moon, Dawn | Botanical elegance across three variants |
| **Everforest** | Dark, Light | Warm green forest tones |
| **Tokyo Night** | Default, Storm | Neo-Tokyo neon aesthetic |
| **Dracula** | — | The classic dark syntax theme |
| **Nord** | — | Arctic, north-bluish clean |
| **Gruvbox** | Dark | Retro groove with warm contrast |
| **One Dark** | — | Atom's iconic syntax palette |
| **Solarized** | Light | Precision colors for machines and people |
| **Kanagawa** | Wave | The great wave off Kanagawa |

Every theme is contract-tested: 40+ semantic tokens, 18 required styles, 5 required gradients.

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
let path = theme.span("file_path", "src/lib.rs");
let hash = theme.span("commit_hash", "a1b2c3d");
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
    println!("{} ({:?}) by {}", info.display_name, info.variant,
             info.author.as_deref().unwrap_or("—"));
}

// Hot-swap themes at runtime
let dracula = load_by_name("dracula").unwrap();
let nord = load_by_name("nord").unwrap();
```

### ThemeBuilder (Programmatic)

```rust
use opaline::ThemeBuilder;

let theme = ThemeBuilder::new("My Theme")
    .palette("bg", "#1a1b26")
    .palette("fg", "#c0caf5")
    .palette("blue", "#7aa2f7")
    .token("text.primary", "fg")
    .token("bg.base", "bg")
    .token("accent.primary", "blue")
    .style("keyword", "accent.primary", None, true, false, false)
    .build()
    .expect("valid theme");
```

## 🪄 Custom Themes

Drop a `.toml` file in `src/builtins/` — it's auto-discovered at compile time. Or load from any path at runtime.

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
"accent.primary" = "blue"
# ... 40+ tokens across text.*, bg.*, accent.*, git.*, diff.*, code.*, etc.

[styles]
keyword = { fg = "accent.primary", bold = true }
# ... 18 required styles

[gradients]
primary = ["blue", "purple"]
# ... 5 required gradients
```

The resolver validates everything at load time — circular references, missing tokens, and invalid colors all produce clear error messages via `OpalineError`.

## ⚙️ Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `builtin-themes` | ✓ | 20 embedded TOML themes via `include_str!` |
| `gradients` | ✓ | Multi-stop gradient interpolation |
| `ratatui` | ✓ | `From` impls, inherent `span()`/`line()`/`text()`/`gradient_text()` |
| `cli` | — | `colored` crate adapter for ANSI output |
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
cargo test --all-features                  # Full test suite (135 tests)
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
