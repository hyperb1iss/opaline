# Opaline

A token-based theme engine for Ratatui TUI applications.

## Architecture

**Resolution pipeline:** TOML file → `ThemeFile` (serde) → `Resolver` (palette → tokens → styles → gradients) → `Theme`

```
src/
  lib.rs              # Re-exports, crate docs
  color.rs            # OpalineColor { r, g, b } + hex parsing + lerp
  style.rs            # OpalineStyle { fg, bg, bold, italic, underline, dim } + merge
  gradient.rs         # Gradient (multi-stop) + at(t) + generate(n)
  error.rs            # OpalineError enum (thiserror)
  schema.rs           # ThemeFile, ThemeMeta, ThemeVariant, StyleDef (serde)
  resolver.rs         # Palette → token → style resolution with cycle detection
  loader.rs           # load_from_file(), load_from_str()
  theme.rs            # Theme struct + accessors + global state
  discovery.rs        # User theme directory scanning
  builtins/
    mod.rs            # include_str!() registry + load_by_name()
    *.toml            # Builtin theme files
  adapters/
    mod.rs
    ratatui.rs        # From<OpalineColor> for Color, gradient_spans()
tests/
  color_tests.rs      # Hex parsing, lerp, Display, FromStr
  style_tests.rs      # Builder, merge
  gradient_tests.rs   # Interpolation, generate, edge cases
  resolver_tests.rs   # Pipeline, cycle detection, fallbacks
  loader_tests.rs     # TOML loading, error paths
  builtins_tests.rs   # Theme validation, token contract
  adapter_tests.rs    # Ratatui From impls, ThemeRatatuiExt
```

## Development Commands

```bash
cargo check                              # Fast type check
cargo clippy --all-targets --all-features # Pedantic lint gate
cargo test --all-features                 # Full test suite
cargo doc --all-features --open           # Generate docs
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `builtin-themes` | yes | Embed TOML themes via include_str! |
| `gradients` | yes | Multi-stop gradient support |
| `ratatui` | yes | From impls for ratatui types |
| `global-state` | no | Process-wide current()/set_theme() |
| `discovery` | no | Load user themes from ~/.config/ |
| `named-colors` | no | CSS named color parsing (not yet implemented) |

## Key Types

- `OpalineColor` — RGB color with hex parsing and lerp
- `OpalineStyle` — Composed style (fg, bg, modifiers) with builder pattern
- `Gradient` — Multi-stop color interpolation
- `Theme` — Fully resolved theme with `color()`, `style()`, `gradient()` accessors
- `OpalineError` — All error variants (IO, Parse, InvalidColor, CircularReference, etc.)

## Conventions

- Edition 2024, MSRV 1.85
- MIT OR Apache-2.0 (Ratatui ecosystem standard)
- `unsafe_code = "forbid"`, `clippy::pedantic` deny
- Tests in `tests/` directory, not inline `#[cfg(test)]`
- `thiserror` for error types
- No `.unwrap()` in library code (`.expect()` allowed for compile-time-verified builtins)

## Token Contract

Every builtin theme must define 40+ semantic tokens across these namespaces:
`text.*`, `bg.*`, `accent.*`, `success/error/warning/info`, `git.*`, `diff.*`,
`border.*`, `code.*`, `mode.*`, `chat.*`
