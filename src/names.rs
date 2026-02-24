//! String constants for the standard theme contract.
//!
//! Every builtin theme defines these tokens, styles, and gradients.
//! Use these instead of raw strings for autocomplete and typo prevention.
//!
//! ```rust
//! use opaline::{Theme, names::tokens, names::styles};
//!
//! let theme = Theme::default();
//! let accent = theme.color(tokens::ACCENT_PRIMARY);
//! let kw = theme.style(styles::KEYWORD);
//! ```

/// Semantic color token names (36 required).
pub mod tokens {
    pub const TEXT_PRIMARY: &str = "text.primary";
    pub const TEXT_SECONDARY: &str = "text.secondary";
    pub const TEXT_MUTED: &str = "text.muted";
    pub const TEXT_DIM: &str = "text.dim";

    pub const BG_BASE: &str = "bg.base";
    pub const BG_PANEL: &str = "bg.panel";
    pub const BG_CODE: &str = "bg.code";
    pub const BG_HIGHLIGHT: &str = "bg.highlight";

    pub const ACCENT_PRIMARY: &str = "accent.primary";
    pub const ACCENT_SECONDARY: &str = "accent.secondary";
    pub const ACCENT_TERTIARY: &str = "accent.tertiary";
    pub const ACCENT_DEEP: &str = "accent.deep";

    pub const SUCCESS: &str = "success";
    pub const ERROR: &str = "error";
    pub const WARNING: &str = "warning";
    pub const INFO: &str = "info";

    pub const GIT_STAGED: &str = "git.staged";
    pub const GIT_MODIFIED: &str = "git.modified";
    pub const GIT_UNTRACKED: &str = "git.untracked";
    pub const GIT_DELETED: &str = "git.deleted";

    pub const DIFF_ADDED: &str = "diff.added";
    pub const DIFF_REMOVED: &str = "diff.removed";
    pub const DIFF_HUNK: &str = "diff.hunk";
    pub const DIFF_CONTEXT: &str = "diff.context";

    pub const BORDER_FOCUSED: &str = "border.focused";
    pub const BORDER_UNFOCUSED: &str = "border.unfocused";

    pub const CODE_HASH: &str = "code.hash";
    pub const CODE_PATH: &str = "code.path";
    pub const CODE_KEYWORD: &str = "code.keyword";
    pub const CODE_FUNCTION: &str = "code.function";
    pub const CODE_STRING: &str = "code.string";
    pub const CODE_NUMBER: &str = "code.number";
    pub const CODE_COMMENT: &str = "code.comment";
    pub const CODE_TYPE: &str = "code.type";
    pub const CODE_LINE_NUMBER: &str = "code.line_number";

    pub const MODE_ACTIVE: &str = "mode.active";
    pub const MODE_INACTIVE: &str = "mode.inactive";
    pub const MODE_HOVER: &str = "mode.hover";

    pub const BG_SELECTION: &str = "bg.selection";
}

/// Named style constants (18 required).
pub mod styles {
    pub const KEYWORD: &str = "keyword";
    pub const FILE_PATH: &str = "file_path";
    pub const COMMIT_HASH: &str = "commit_hash";
    pub const SELECTED: &str = "selected";
    pub const ACTIVE_SELECTED: &str = "active_selected";
    pub const FOCUSED_BORDER: &str = "focused_border";
    pub const UNFOCUSED_BORDER: &str = "unfocused_border";
    pub const SUCCESS_STYLE: &str = "success_style";
    pub const ERROR_STYLE: &str = "error_style";
    pub const WARNING_STYLE: &str = "warning_style";
    pub const INFO_STYLE: &str = "info_style";
    pub const DIMMED: &str = "dimmed";
    pub const MUTED: &str = "muted";
    pub const INLINE_CODE: &str = "inline_code";
    pub const GIT_STAGED: &str = "git_staged";
    pub const GIT_MODIFIED: &str = "git_modified";
    pub const DIFF_ADDED: &str = "diff_added";
    pub const DIFF_REMOVED: &str = "diff_removed";
    pub const DIFF_HUNK: &str = "diff_hunk";
    pub const DIFF_CONTEXT: &str = "diff_context";
    pub const LINE_NUMBER: &str = "line_number";
    pub const TIMESTAMP: &str = "timestamp";
    pub const AUTHOR: &str = "author";
    pub const GIT_UNTRACKED: &str = "git_untracked";
    pub const GIT_DELETED: &str = "git_deleted";
    pub const MODE_INACTIVE: &str = "mode_inactive";
}

/// Named gradient constants (5 required).
pub mod gradients {
    pub const PRIMARY: &str = "primary";
    pub const WARM: &str = "warm";
    pub const SUCCESS_GRADIENT: &str = "success_gradient";
    pub const ERROR_GRADIENT: &str = "error_gradient";
    pub const AURORA: &str = "aurora";
}
