# Tokens

Tokens are the **semantic layer** between raw palette colors and composed styles. They give meaning to colors — `accent.primary` means "the main accent color" regardless of whether it's purple, blue, or green.

## Token Namespaces

Opaline's token contract defines 40+ tokens across 10 namespaces:

### Text

| Token | Purpose |
|-------|---------|
| `text.primary` | Main body text |
| `text.secondary` | Less prominent text |
| `text.muted` | De-emphasized text |
| `text.dim` | Very subtle text (line numbers, metadata) |

### Background

| Token | Purpose |
|-------|---------|
| `bg.base` | Main background |
| `bg.panel` | Panel/sidebar background |
| `bg.code` | Code block background |
| `bg.highlight` | Highlighted line background |

### Accent

| Token | Purpose |
|-------|---------|
| `accent.primary` | Main accent (keywords, active states) |
| `accent.secondary` | Secondary accent (functions, links) |
| `accent.tertiary` | Third accent (types, special elements) |
| `accent.deep` | Deep/saturated accent for emphasis |

### Status

| Token | Purpose |
|-------|---------|
| `success` | Success states |
| `error` | Error states |
| `warning` | Warning states |
| `info` | Informational states |

### Git

| Token | Purpose |
|-------|---------|
| `git.staged` | Staged changes |
| `git.modified` | Modified files |
| `git.untracked` | Untracked files |
| `git.deleted` | Deleted files |

### Diff

| Token | Purpose |
|-------|---------|
| `diff.added` | Added lines |
| `diff.removed` | Removed lines |
| `diff.hunk` | Hunk headers |
| `diff.context` | Context lines |

### Border

| Token | Purpose |
|-------|---------|
| `border.focused` | Focused panel border |
| `border.unfocused` | Unfocused panel border |

### Code

| Token | Purpose |
|-------|---------|
| `code.hash` | Commit hashes, hex values |
| `code.path` | File paths |
| `code.keyword` | Language keywords |
| `code.function` | Function names |
| `code.string` | String literals |
| `code.number` | Numeric literals |
| `code.comment` | Comments |
| `code.type` | Type names |
| `code.line_number` | Line numbers |

### Mode

| Token | Purpose |
|-------|---------|
| `mode.active` | Active mode indicator |
| `mode.inactive` | Inactive mode indicator |
| `mode.hover` | Hovered mode indicator |

### Chat

| Token | Purpose |
|-------|---------|
| `chat.user` | User messages |
| `chat.iris` | AI/assistant messages |

## Accessing Tokens

```rust
let theme = opaline::Theme::default();

// Get a resolved color (falls back to magenta if missing)
let color = theme.color("accent.primary");

// Strict lookup — None if missing
let color = theme.try_color("accent.primary");

// Check existence
if theme.has_token("accent.primary") {
    // ...
}

// List all token names
let names = theme.token_names();
```

## Token Resolution

Tokens resolve through the palette. A token value is a palette key:

```toml
[palette]
electric_purple = "#e135ff"

[tokens]
"accent.primary" = "electric_purple"  # → OpalineColor(225, 53, 255)
```

Tokens can also reference other tokens (transitive resolution):

```toml
[tokens]
"accent.primary" = "electric_purple"
"code.keyword" = "accent.primary"     # → also resolves to #e135ff
```

The resolver detects circular references and reports them as errors.
