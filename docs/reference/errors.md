# Error Reference

Opaline uses a single error enum for all failure modes. All errors implement `std::error::Error` via `thiserror`.

## `OpalineError`

```rust
use opaline::OpalineError;
```

### Variants

| Variant | When | Example |
|---------|------|---------|
| `Io` | File operations fail | File not found, permission denied |
| `TomlParse` | TOML syntax error | Missing closing quote, invalid table |
| `InvalidColor` | Hex string isn't a valid color | `"#xyz"`, `"not-a-color"` |
| `UnresolvableToken` | Token references unknown palette/token | `"accent.primary" = "nonexistent"` |
| `CircularReference` | Tokens form a cycle | `a → b → c → a` |
| `EmptyGradient` | Gradient has no stops | `gradient = []` |

### Handling

All public functions that can fail return `Result<T, OpalineError>`:

```rust
use opaline::OpalineError;

match opaline::load_from_file("theme.toml") {
    Ok(theme) => println!("Loaded: {}", theme.meta.name),
    Err(OpalineError::Io(e)) => eprintln!("File error: {e}"),
    Err(OpalineError::TomlParse(e)) => eprintln!("TOML syntax: {e}"),
    Err(OpalineError::InvalidColor { value }) => {
        eprintln!("Bad color: {value}");
    }
    Err(OpalineError::UnresolvableToken { name, reference }) => {
        eprintln!("Token '{name}' references unknown '{reference}'");
    }
    Err(OpalineError::CircularReference { chain }) => {
        eprintln!("Circular: {}", chain.join(" → "));
    }
    Err(e) => eprintln!("Other: {e}"),
}
```

### Strict by Design

Opaline's resolver is intentionally strict:

- **No silent fallbacks** — If a token can't resolve, you get an error, not a default color
- **Cycle detection** — The resolver tracks the reference chain and errors on cycles
- **Early failure** — All validation happens at load time, not when you access a color

This means a theme that loads successfully is guaranteed to have all its references resolved correctly.

::: tip
For fallback-safe access at runtime, use the non-strict methods:
- `theme.color("token")` — returns magenta fallback on miss
- `theme.style("name")` — returns default style on miss
- `theme.gradient("name", t)` — returns magenta fallback on miss

For strict access that returns `Option`:
- `theme.try_color("token")` — returns `None` on miss
- `theme.try_style("name")` — returns `None` on miss
- `theme.try_gradient("name", t)` — returns `None` on miss
:::
