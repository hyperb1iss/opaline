# Opaline

A token-based theme engine for Ratatui TUI applications.

## Architecture

**Resolution pipeline:** TOML file → `ThemeFile` (serde) → `Resolver` (palette → tokens → styles → gradients) → `Theme`

```
src/
  lib.rs              # Re-exports, crate docs
  color.rs            # OpalineColor { r, g, b } + hex parsing + lerp + From conversions
  style.rs            # OpalineStyle { fg, bg, bold, italic, underline, dim } + merge
  gradient.rs         # Gradient (multi-stop) + at(t) + generate(n) + try_new()
  error.rs            # OpalineError enum (thiserror)
  schema.rs           # ThemeFile, ThemeMeta, ThemeVariant, StyleDef (serde)
  resolver.rs         # Strict palette → token → style resolution with cycle detection
  loader.rs           # load_from_file(), load_from_str()
  theme.rs            # Theme struct + accessors + ThemeBuilder + global state
  discovery.rs        # User theme directory scanning (app-specific)
  builtins/
    mod.rs            # Auto-generated registry via build.rs
    *.toml            # 13 builtin theme files (auto-discovered at compile time)
  adapters/
    mod.rs
    ratatui.rs        # From<OpalineColor> for Color, ThemeRatatuiExt, gradient_spans()
    cli.rs            # colored crate: ColoredExt, ThemeCliExt, gradient_string()
build.rs              # Compile-time theme auto-discovery and codegen
tests/
  color_tests.rs      # Hex parsing, lerp, Display, FromStr
  style_tests.rs      # Builder, merge
  gradient_tests.rs   # Interpolation, generate, edge cases
  resolver_tests.rs   # Pipeline, cycle detection, strict error paths
  loader_tests.rs     # TOML loading, error paths
  builtins_tests.rs   # Theme validation, token/style/gradient contracts
  adapter_tests.rs    # Ratatui From impls, ThemeRatatuiExt
```

## Development Commands

```bash
cargo check                              # Fast type check
cargo clippy --all-targets --all-features # Pedantic lint gate
cargo test --all-features                 # Full test suite (99 tests)
cargo doc --all-features --open           # Generate docs
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `builtin-themes` | yes | 13 embedded TOML themes via include_str! |
| `gradients` | yes | Multi-stop gradient support |
| `ratatui` | yes | From impls for ratatui types |
| `cli` | no | colored crate adapter for ANSI terminal output |
| `global-state` | no | Process-wide current()/set_theme() |
| `discovery` | no | Load user themes from ~/.config/ |
| `named-colors` | no | CSS named color parsing (not yet implemented) |

## Key Types

- `OpalineColor` — RGB color with hex, tuple, array, u32 conversions + lerp
- `OpalineStyle` — Composed style (fg, bg, modifiers) with builder pattern
- `Gradient` — Multi-stop color interpolation (new() panics, try_new() returns Result)
- `Theme` — Fully resolved theme with `color()`, `style()`, `gradient()` accessors
- `ThemeBuilder` — Programmatic theme construction without TOML
- `ThemeInfo` — Metadata for theme discovery and picker UIs
- `OpalineError` — All error variants (IO, Parse, InvalidColor, CircularReference, etc.)

## Conventions

- Edition 2024, MSRV 1.85
- MIT OR Apache-2.0 (Ratatui ecosystem standard)
- `unsafe_code = "forbid"`, `clippy::pedantic` deny
- Tests in `tests/` directory, not inline `#[cfg(test)]`
- `thiserror` for error types
- No `.unwrap()` in library code (`.expect()` allowed for compile-time-verified builtins)
- Strict resolver: unresolvable tokens, style refs, gradient stops error immediately

## Token Contract

Every builtin theme must define 40+ semantic tokens across these namespaces:
`text.*`, `bg.*`, `accent.*`, `success/error/warning/info`, `git.*`, `diff.*`,
`border.*`, `code.*`, `mode.*`, `chat.*`

Plus 18 required styles, 5 required gradients — enforced by contract tests.

## Adding a Builtin Theme

Drop a `.toml` file in `src/builtins/`. It's auto-discovered at compile time.
Use underscores in filenames (e.g. `rose_pine.toml` → id: `rose-pine`).
Must pass all contract tests (tokens, styles, gradients).
