# Token Contract

Every builtin theme must define a minimum set of semantic tokens, styles, and gradients. This contract ensures that consuming applications can rely on these names existing in any theme.

## Required Tokens (40)

These tokens must be present in every builtin theme:

### Text (4)

```
text.primary
text.secondary
text.muted
text.dim
```

### Background (4)

```
bg.base
bg.panel
bg.code
bg.highlight
```

### Accent (4)

```
accent.primary
accent.secondary
accent.tertiary
accent.deep
```

### Status (4)

```
success
error
warning
info
```

### Git (4)

```
git.staged
git.modified
git.untracked
git.deleted
```

### Diff (4)

```
diff.added
diff.removed
diff.hunk
diff.context
```

### Border (2)

```
border.focused
border.unfocused
```

### Code (9)

```
code.hash
code.path
code.keyword
code.function
code.string
code.number
code.comment
code.type
code.line_number
```

### Mode (3)

```
mode.active
mode.inactive
mode.hover
```

### Chat (2)

```
chat.user
chat.iris
```

## Required Styles (18)

```
keyword
file_path
commit_hash
selected
active_selected
focused_border
unfocused_border
success_style
error_style
warning_style
info_style
dimmed
muted
inline_code
git_staged
git_modified
diff_added
diff_removed
```

## Required Gradients (5)

```
primary
warm
success_gradient
error_gradient
aurora
```

## Enforcement

The contract is enforced by integration tests in `tests/builtins_tests.rs`. Every builtin theme is loaded and checked for all required tokens, styles, and gradients.

```rust
// From builtins_tests.rs
#[test]
fn all_builtins_have_required_tokens() {
    for &(id, _) in builtins::builtin_names() {
        let theme = builtins::load_by_name(id).expect("loads");
        for &token in REQUIRED_TOKENS {
            assert!(
                theme.has_token(token),
                "theme '{id}' missing required token: {token}"
            );
        }
    }
}
```

## Adding Tokens to Your Theme

If you're creating a custom theme, you don't need to satisfy the full contract — it's only enforced for builtins. However, following the contract ensures your theme works with any Opaline-powered app.

Use the [custom themes template](../guide/custom-themes) as a starting point.
