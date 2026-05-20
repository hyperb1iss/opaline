#![cfg(feature = "widgets")]

use std::sync::{Mutex, MutexGuard, OnceLock};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use opaline::widgets::wrap_text;
use opaline::{Theme, ThemeSelectorAction, ThemeSelectorState, current, set_theme};

fn global_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().expect("lock")
}

#[test]
fn j_and_k_are_search_input_not_navigation() {
    let _guard = global_lock();
    let previous = current();

    let mut state = ThemeSelectorState::new();

    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE)),
        ThemeSelectorAction::FilterChanged
    );
    assert_eq!(state.filter(), "j");

    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)),
        ThemeSelectorAction::FilterChanged
    );
    assert_eq!(state.filter(), "jk");

    set_theme((*previous).clone());
}

#[test]
fn esc_restores_original_theme_snapshot() {
    let _guard = global_lock();
    let previous = current();

    let original = Theme::builder("Original Snapshot").build();
    let preview = Theme::builder("Preview Theme").build();

    set_theme(original);
    let mut state = ThemeSelectorState::new();

    set_theme(preview);
    assert_eq!(current().meta.name, "Preview Theme");

    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        ThemeSelectorAction::Cancel
    );
    assert_eq!(current().meta.name, "Original Snapshot");

    set_theme((*previous).clone());
}

// ── wrap_text unit tests ────────────────────────────────────────────────

#[test]
fn wrap_text_splits_at_word_boundary() {
    let result = wrap_text("hello world foo", 12);
    assert_eq!(result, vec!["hello world", "foo"]);
}

#[test]
fn wrap_text_returns_empty_for_blank_input() {
    let result = wrap_text("", 20);
    assert!(result.is_empty());
}

#[test]
fn wrap_text_passes_overlong_word_through_unbroken() {
    let result = wrap_text("superlongword", 5);
    assert_eq!(result, vec!["superlongword"]);
}

#[test]
fn wrap_text_uses_display_width_not_byte_length() {
    let result = wrap_text("é é", 3);
    assert_eq!(result, vec!["é é"]);
}
