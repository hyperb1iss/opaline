# Contributing to Opaline

Thanks for your interest in contributing! Whether it's a bug report, new theme, or code improvement — it all helps.

## Adding a Theme

The easiest way to contribute — drop a `.toml` file in `src/builtins/`:

1. Copy an existing theme as a starting point
2. Fill in `[meta]`, `[palette]`, `[tokens]`, `[styles]`, `[gradients]`
3. Run `cargo test --all-features` — the contract tests enforce 40+ tokens, 18 styles, 5 gradients
4. Open a PR

Use underscores in filenames (e.g., `my_theme.toml` becomes id `my-theme`). Themes are auto-discovered at compile time via `build.rs`.

## Development Setup

### Prerequisites

- **Rust 1.85+** (edition 2024) — install via [rustup](https://rustup.rs/)
- **just** (optional) — `cargo install just` for the task runner

### Build & Test

```bash
git clone https://github.com/hyperb1iss/opaline.git
cd opaline
cargo build --all-features
cargo test --all-features
```

Or with just:

```bash
just check     # fmt + clippy
just test-all  # nextest + doc tests
just demo      # interactive theme showcase
just ci        # full pipeline (check + test + deny)
```

## Code Style

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Key rules: `unsafe_code = "forbid"`, `clippy::pedantic` deny, `unwrap_used = "deny"`.

## Pull Request Workflow

1. **Fork** the repository
2. **Branch** from `main`
3. **Implement** your changes with tests where applicable
4. **Ensure CI passes** locally: `just ci` or `cargo fmt --all --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features`
5. **Open a PR** targeting `main`

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 License.
