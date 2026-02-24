# Installation

## Add to Your Project

```bash
cargo add opaline
```

This enables the default features: `builtin-themes`, `gradients`, and `ratatui`.

## Feature Flags

Opaline uses feature flags to keep the dependency tree lean. Enable only what you need:

```toml
[dependencies]
# Default — builtins + gradients + ratatui adapter
opaline = "0.1"

# Minimal — just the core theme engine
opaline = { version = "0.1", default-features = false }

# With CLI colored output
opaline = { version = "0.1", features = ["cli"] }

# With global theme singleton
opaline = { version = "0.1", features = ["global-state"] }

# Everything
opaline = { version = "0.1", features = ["builtin-themes", "gradients", "ratatui", "cli", "global-state", "discovery"] }
```

| Feature | Default | Description |
|---------|---------|-------------|
| `builtin-themes` | yes | 20 embedded TOML themes via `include_str!` |
| `gradients` | yes | Multi-stop gradient support |
| `ratatui` | yes | `From` impls for `ratatui::style::{Color, Style}` |
| `cli` | no | `colored` crate adapter for ANSI terminal output |
| `global-state` | no | Process-wide `current()`/`set_theme()` singleton |
| `discovery` | no | Load user themes from `~/.config/<app>/themes/` |

## Requirements

- **Rust edition 2024** (MSRV 1.85)
- `ratatui-core` 0.1 (pulled automatically when `ratatui` feature is enabled)

## Verify Installation

```rust
use opaline::Theme;

fn main() {
    let theme = Theme::default();
    println!("Loaded: {} ({})", theme.meta.name,
        if theme.is_dark() { "dark" } else { "light" });
}
```

This should print: `Loaded: SilkCircuit Neon (dark)`
