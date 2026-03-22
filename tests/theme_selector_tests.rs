#![cfg(feature = "widgets")]

use std::sync::{Mutex, MutexGuard, OnceLock};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
