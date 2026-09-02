#![cfg(feature = "widgets")]

use std::sync::{Mutex, MutexGuard, OnceLock};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

use opaline::widgets::wrap_text;
use opaline::{Theme, ThemeSelector, ThemeSelectorAction, ThemeSelectorState, current, set_theme};

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

#[test]
fn release_events_are_ignored() {
    let _guard = global_lock();
    let previous = current();

    let mut state = ThemeSelectorState::new();
    let release = KeyEvent::new_with_kind(
        KeyCode::Char('a'),
        KeyModifiers::NONE,
        KeyEventKind::Release,
    );

    assert_eq!(state.handle_key(release), ThemeSelectorAction::Noop);
    assert_eq!(state.filter(), "");

    set_theme((*previous).clone());
}

#[test]
fn control_chords_do_not_enter_the_filter() {
    let _guard = global_lock();
    let previous = current();

    let mut state = ThemeSelectorState::new();

    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        ThemeSelectorAction::Noop
    );
    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::ALT)),
        ThemeSelectorAction::Noop
    );
    assert_eq!(state.filter(), "");

    // Shift is just an uppercase character, which is still search input.
    assert_eq!(
        state.handle_key(KeyEvent::new(KeyCode::Char('N'), KeyModifiers::SHIFT)),
        ThemeSelectorAction::FilterChanged
    );
    assert_eq!(state.filter(), "N");

    set_theme((*previous).clone());
}

#[test]
fn enter_without_navigation_applies_the_reported_theme() {
    let _guard = global_lock();
    let previous = current();

    set_theme(opaline::load_by_name("nord").expect("nord loads"));
    let mut state = ThemeSelectorState::new();
    let expected = state.selected_theme().expect("list is not empty").clone();

    let action = state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(action, ThemeSelectorAction::Select(expected.name.clone()));
    assert_eq!(current().meta.name, expected.display_name);

    set_theme((*previous).clone());
}

#[test]
fn scrolled_list_shows_the_heading_for_the_visible_section() {
    let _guard = global_lock();
    let previous = current();

    let mut state = ThemeSelectorState::new();
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    }

    let area = Rect::new(0, 0, 80, 12);
    let mut buf = Buffer::empty(area);
    ThemeSelector::new().render(area, &mut buf, &mut state);

    let rows: Vec<String> = (0..area.height)
        .map(|y| (0..area.width).map(|x| buf[(x, y)].symbol()).collect())
        .collect();

    assert!(
        rows.iter().any(|row| row.contains("Light Themes")),
        "expected the light heading in {rows:#?}"
    );
    assert!(
        !rows.iter().any(|row| row.contains("Dark Themes")),
        "dark heading should not be visible in {rows:#?}"
    );

    set_theme((*previous).clone());
}

#[test]
fn section_heading_is_never_the_last_list_row() {
    let _guard = global_lock();
    let previous = current();

    let dark_count = opaline::list_available_themes()
        .iter()
        .filter(|info| info.variant == opaline::ThemeVariant::Dark)
        .count();
    let dark_count = u16::try_from(dark_count).expect("fits in u16");

    // Sweep heights around the point where the light section's heading
    // lands on the final visible row. Whatever the chrome costs, one of
    // these heights hits it, and a heading with nothing beneath it is the
    // defect under test.
    for height in (dark_count + 2)..=(dark_count + 14) {
        let mut state = ThemeSelectorState::new();
        let area = Rect::new(0, 0, 80, height);
        let mut buf = Buffer::empty(area);
        ThemeSelector::new().render(area, &mut buf, &mut state);

        let rows: Vec<String> = (0..area.height)
            .map(|y| (0..area.width).map(|x| buf[(x, y)].symbol()).collect())
            .collect();

        if let Some(i) = rows.iter().position(|row| row.contains("Light Themes")) {
            let below = rows.get(i + 1).map_or("", String::as_str);
            assert!(
                below.contains('\u{2600}'),
                "height {height}: light heading at row {i} has no light entry beneath it:\n{rows:#?}"
            );
        }
    }

    set_theme((*previous).clone());
}
