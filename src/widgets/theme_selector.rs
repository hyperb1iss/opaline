//! A reusable theme selector widget with live preview.
//!
//! Provides a searchable, scrollable list of all available themes with a
//! color preview pane. Integrates with Opaline's global state for instant
//! live preview on navigation, and supports exact rollback on cancel.
//!
//! # Usage
//!
//! ```rust,ignore
//! // Open the selector
//! let state = ThemeSelectorState::with_current_selected()
//!     .with_derive(my_app::derive_tokens);
//!
//! // In your key handler
//! match state.handle_key(key) {
//!     ThemeSelectorAction::Select(id) => save_preference(&id),
//!     ThemeSelectorAction::Cancel => { /* original theme restored */ },
//!     _ => {}
//! }
//!
//! // In your render
//! frame.render_stateful_widget(ThemeSelector::new(), area, &mut state);
//! ```

use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};

use crate::builtins::ThemeInfo;
use crate::names::tokens;
use crate::theme::Theme;
use crate::{ThemeVariant, current, list_available_themes, set_theme};

// ── Action ────────────────────────────────────────────────────────────

/// Result of handling a key event in the theme selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeSelectorAction {
    /// Cursor moved — live preview applied.
    Navigate,
    /// Enter pressed — theme confirmed. Contains the theme's kebab-case ID.
    Select(String),
    /// Esc pressed — original theme restored.
    Cancel,
    /// Filter text changed — list recomputed.
    FilterChanged,
    /// Key not handled by the selector.
    Noop,
}

// ── State ─────────────────────────────────────────────────────────────

/// Owned state for the theme selector. Create on open, drop on close.
pub struct ThemeSelectorState {
    /// All available themes (sorted: dark first, then alpha).
    themes: Vec<ThemeInfo>,
    /// Pre-loaded themes for instant preview (indices match `themes`).
    theme_cache: Vec<Theme>,
    /// Lowercase `(display_name, author)` pairs for fast filtering.
    search_cache: Vec<(String, String)>,
    /// Current filter text.
    filter: String,
    /// Indices into `themes` that match the current filter.
    filtered_indices: Vec<usize>,
    /// Cursor position within `filtered_indices`.
    cursor: usize,
    /// Scroll offset for the visible list.
    scroll: usize,
    /// Snapshot of the theme at open time, for Esc restore.
    original_theme: Arc<Theme>,
    /// Optional app-level token derivation callback.
    derive_fn: Option<fn(&mut Theme)>,
}

impl ThemeSelectorState {
    /// Create a new selector state, populating from all available themes.
    ///
    /// Themes are sorted dark-first, then alphabetically within each variant.
    /// All themes are pre-loaded into memory for instant preview.
    pub fn new() -> Self {
        let mut themes = list_available_themes();

        // Sort: dark themes first, then alphabetically by display name
        themes.sort_by(|a, b| {
            let variant_ord = match (&a.variant, &b.variant) {
                (ThemeVariant::Dark, ThemeVariant::Light) => std::cmp::Ordering::Less,
                (ThemeVariant::Light, ThemeVariant::Dark) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            };
            variant_ord.then_with(|| a.display_name.cmp(&b.display_name))
        });

        // Pre-load all themes into cache
        let theme_cache: Vec<Theme> = themes
            .iter()
            .map(|info| {
                crate::builtins::load_by_name(&info.name)
                    .or_else(|| {
                        info.path
                            .as_ref()
                            .and_then(|p| crate::load_from_file(p).ok())
                    })
                    .unwrap_or_default()
            })
            .collect();

        let search_cache: Vec<(String, String)> = themes
            .iter()
            .map(|info| (info.display_name.to_lowercase(), info.author.to_lowercase()))
            .collect();

        let filtered_indices: Vec<usize> = (0..themes.len()).collect();
        let original_theme = current();

        Self {
            themes,
            theme_cache,
            search_cache,
            filter: String::new(),
            filtered_indices,
            cursor: 0,
            scroll: 0,
            original_theme,
            derive_fn: None,
        }
    }

    /// Create a new selector with the currently active theme pre-selected.
    pub fn with_current_selected() -> Self {
        let mut state = Self::new();
        let current_name = state.original_theme.meta.name.clone();

        // Find the index matching the current theme
        if let Some(pos) = state
            .themes
            .iter()
            .position(|info| info.display_name == current_name || info.name == current_name)
        {
            if let Some(cursor_pos) = state.filtered_indices.iter().position(|&i| i == pos) {
                state.cursor = cursor_pos;
                // Center the cursor in the viewport
                state.scroll = cursor_pos.saturating_sub(8);
            }
        }

        state
    }

    /// Set the app-level token derivation callback for live preview.
    ///
    /// This function is called on each cached theme clone before it's applied
    /// as the global theme, allowing apps to register their derived tokens.
    #[must_use]
    pub fn with_derive(mut self, derive: fn(&mut Theme)) -> Self {
        self.derive_fn = Some(derive);
        self
    }

    /// Handle a key event. Returns the action taken.
    pub fn handle_key(&mut self, key: KeyEvent) -> ThemeSelectorAction {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.filtered_indices.is_empty() {
                    return ThemeSelectorAction::Noop;
                }
                if self.cursor > 0 {
                    self.cursor -= 1;
                    if self.cursor < self.scroll {
                        self.scroll = self.cursor;
                    }
                    self.apply_preview();
                    ThemeSelectorAction::Navigate
                } else {
                    ThemeSelectorAction::Noop
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.filtered_indices.is_empty() {
                    return ThemeSelectorAction::Noop;
                }
                if self.cursor + 1 < self.filtered_indices.len() {
                    self.cursor += 1;
                    // Scroll adjustment happens in render via clamp_scroll
                    self.apply_preview();
                    ThemeSelectorAction::Navigate
                } else {
                    ThemeSelectorAction::Noop
                }
            }
            KeyCode::Enter => {
                if let Some(&idx) = self.filtered_indices.get(self.cursor) {
                    let id = self.themes[idx].name.clone();
                    // Theme is already applied as preview — just confirm
                    ThemeSelectorAction::Select(id)
                } else {
                    ThemeSelectorAction::Noop
                }
            }
            KeyCode::Esc => {
                // Restore the original theme exactly
                set_theme((*self.original_theme).clone());
                ThemeSelectorAction::Cancel
            }
            KeyCode::Char(c) => {
                self.filter.push(c);
                self.recompute_filter();
                self.apply_preview();
                ThemeSelectorAction::FilterChanged
            }
            KeyCode::Backspace => {
                if self.filter.pop().is_some() {
                    self.recompute_filter();
                    self.apply_preview();
                    ThemeSelectorAction::FilterChanged
                } else {
                    ThemeSelectorAction::Noop
                }
            }
            _ => ThemeSelectorAction::Noop,
        }
    }

    /// The currently selected theme info, if any.
    pub fn selected_theme(&self) -> Option<&ThemeInfo> {
        self.filtered_indices
            .get(self.cursor)
            .map(|&idx| &self.themes[idx])
    }

    /// Current filter text.
    pub fn filter(&self) -> &str {
        &self.filter
    }

    // ── Internal ──────────────────────────────────────────────────────

    fn recompute_filter(&mut self) {
        let query = self.filter.to_lowercase();
        self.filtered_indices = if query.is_empty() {
            (0..self.themes.len()).collect()
        } else {
            self.search_cache
                .iter()
                .enumerate()
                .filter(|(_, (name, author))| name.contains(&query) || author.contains(&query))
                .map(|(i, _)| i)
                .collect()
        };

        // Reset cursor to first match
        self.cursor = 0;
        self.scroll = 0;
    }

    fn apply_preview(&self) {
        if let Some(&idx) = self.filtered_indices.get(self.cursor) {
            let mut theme = self.theme_cache[idx].clone();
            if let Some(derive) = self.derive_fn {
                derive(&mut theme);
            }
            set_theme(theme);
        }
    }

    /// Clamp scroll so cursor is visible within the given viewport height.
    fn clamp_scroll(&mut self, visible_items: usize) {
        if visible_items == 0 {
            return;
        }
        if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
        if self.cursor >= self.scroll + visible_items {
            self.scroll = self.cursor - visible_items + 1;
        }
    }
}

impl Default for ThemeSelectorState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Widget ────────────────────────────────────────────────────────────

/// The theme selector widget. Renders a two-pane view: searchable list +
/// color preview. Pair with [`ThemeSelectorState`].
pub struct ThemeSelector<'a> {
    title: &'a str,
}

impl<'a> ThemeSelector<'a> {
    /// Create a new theme selector widget.
    pub fn new() -> Self {
        Self {
            title: "Select Theme",
        }
    }

    /// Override the default title.
    #[must_use]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

impl Default for ThemeSelector<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for ThemeSelector<'_> {
    type State = ThemeSelectorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ThemeSelectorState) {
        let t = current();

        // Outer block
        let block = Block::default()
            .title(Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    self.title,
                    Style::default()
                        .fg(Color::from(t.color(tokens::TEXT_PRIMARY)))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::from(t.color(tokens::ACCENT_PRIMARY))));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 20 || inner.height < 6 {
            return;
        }

        // 55% list / 45% preview
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(inner);

        render_theme_list(chunks[0], buf, state, &t);
        render_theme_preview(chunks[1], buf, state, &t);
    }
}

// ── List pane ─────────────────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
fn render_theme_list(area: Rect, buf: &mut Buffer, state: &mut ThemeSelectorState, t: &Theme) {
    if area.height < 4 {
        return;
    }

    let accent_primary: Color = t.color(tokens::ACCENT_PRIMARY).into();
    let accent_secondary: Color = t.color(tokens::ACCENT_SECONDARY).into();
    let text_primary: Color = t.color(tokens::TEXT_PRIMARY).into();
    let text_secondary: Color = t.color(tokens::TEXT_SECONDARY).into();
    let text_muted: Color = t.color(tokens::TEXT_MUTED).into();
    let warning: Color = t.color(tokens::WARNING).into();

    // Layout: filter (2) + list (remaining) + hints (1)
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(area);

    // ── Filter input ──────────────────────────────────────────────
    let cursor_char = if state.filter.is_empty() {
        "\u{2502}" // │
    } else {
        "\u{2588}" // █
    };
    let filter_line = Line::from(vec![
        Span::styled(" Filter: ", Style::default().fg(text_muted)),
        Span::styled(&state.filter, Style::default().fg(accent_secondary)),
        Span::styled(cursor_char, Style::default().fg(accent_primary)),
    ]);
    filter_line.render(chunks[0], buf);

    // ── Theme list ────────────────────────────────────────────────
    let list_area = chunks[1];

    // Count section headers to determine visible items accurately
    #[allow(clippy::as_conversions)]
    let mut visible_items = list_area.height as usize;
    let section_headers = count_section_headers(state);
    visible_items = visible_items.saturating_sub(section_headers);

    state.clamp_scroll(visible_items);

    render_theme_entries(
        list_area,
        buf,
        state,
        accent_secondary,
        text_primary,
        text_muted,
        warning,
    );

    // ── Footer hints ──────────────────────────────────────────────
    let total = state.filtered_indices.len();
    let position = if total > 0 {
        format!(" ({}/{})", state.cursor + 1, total)
    } else {
        String::new()
    };
    let hints = Line::from(vec![
        Span::styled(
            " \u{2191}\u{2193} nav  Enter select  Esc cancel",
            Style::default().fg(text_muted),
        ),
        Span::styled(position, Style::default().fg(text_secondary)),
    ]);
    hints.render(chunks[2], buf);
}

/// Render the actual list entries with section headers.
fn render_theme_entries(
    list_area: Rect,
    buf: &mut Buffer,
    state: &ThemeSelectorState,
    accent_secondary: Color,
    text_primary: Color,
    text_muted: Color,
    warning: Color,
) {
    let mut y = list_area.y;
    let max_y = list_area.y + list_area.height;
    let mut items_rendered = 0;
    let mut last_variant: Option<ThemeVariant> = None;

    for (filter_idx, &theme_idx) in state.filtered_indices.iter().enumerate() {
        if y >= max_y {
            break;
        }

        let info = &state.themes[theme_idx];

        // Section header on variant boundary
        if last_variant != Some(info.variant) {
            if (items_rendered >= state.scroll || last_variant.is_none()) && y < max_y {
                let header_text = match info.variant {
                    ThemeVariant::Dark => " Dark Themes",
                    ThemeVariant::Light => " Light Themes",
                };
                let header = Line::from(Span::styled(
                    header_text,
                    Style::default()
                        .fg(text_muted)
                        .add_modifier(Modifier::ITALIC),
                ));
                header.render(Rect::new(list_area.x, y, list_area.width, 1), buf);
                y += 1;
            }
            last_variant = Some(info.variant);
        }

        // Skip items before scroll window
        if items_rendered < state.scroll {
            items_rendered += 1;
            continue;
        }

        if y >= max_y {
            break;
        }

        let is_selected = filter_idx == state.cursor;

        let mut spans = vec![];
        if is_selected {
            spans.push(Span::styled(
                "  > ",
                Style::default()
                    .fg(accent_secondary)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                &info.display_name,
                Style::default()
                    .fg(accent_secondary)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw("    "));
            spans.push(Span::styled(
                &info.display_name,
                Style::default().fg(text_primary),
            ));
        }

        // Light theme indicator
        if info.variant == ThemeVariant::Light {
            spans.push(Span::styled(" \u{2600}", Style::default().fg(warning)));
        }

        Line::from(spans).render(Rect::new(list_area.x, y, list_area.width, 1), buf);

        y += 1;
        items_rendered += 1;
    }
}

/// Count how many section headers will appear in the filtered list.
fn count_section_headers(state: &ThemeSelectorState) -> usize {
    let mut count = 0;
    let mut last_variant: Option<ThemeVariant> = None;
    for &idx in &state.filtered_indices {
        let variant = state.themes[idx].variant;
        if last_variant != Some(variant) {
            count += 1;
            last_variant = Some(variant);
        }
    }
    count
}

// ── Preview pane ──────────────────────────────────────────────────────

#[allow(clippy::too_many_lines)]
fn render_theme_preview(area: Rect, buf: &mut Buffer, state: &ThemeSelectorState, t: &Theme) {
    // Left border separator
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(Color::from(t.color(tokens::TEXT_DIM))));
    let inner = block.inner(area);
    block.render(area, buf);

    if inner.height < 4 || inner.width < 10 {
        return;
    }

    let Some(info) = state
        .filtered_indices
        .get(state.cursor)
        .map(|&idx| &state.themes[idx])
    else {
        let empty = Line::from(Span::styled(
            " No themes match",
            Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
        ));
        empty.render(
            Rect::new(inner.x + 1, inner.y + 1, inner.width.saturating_sub(1), 1),
            buf,
        );
        return;
    };

    let accent_primary: Color = t.color(tokens::ACCENT_PRIMARY).into();
    let text_primary: Color = t.color(tokens::TEXT_PRIMARY).into();
    let text_secondary: Color = t.color(tokens::TEXT_SECONDARY).into();
    let text_muted: Color = t.color(tokens::TEXT_MUTED).into();

    let x = inner.x + 1;
    let w = inner.width.saturating_sub(2);
    let mut y = inner.y + 1;
    let max_y = inner.y + inner.height;

    // Theme name
    if y < max_y {
        Line::from(Span::styled(
            &info.display_name,
            Style::default()
                .fg(accent_primary)
                .add_modifier(Modifier::BOLD),
        ))
        .render(Rect::new(x, y, w, 1), buf);
        y += 1;
    }

    // Author
    if y < max_y && !info.author.is_empty() {
        Line::from(vec![
            Span::styled("by ", Style::default().fg(text_muted)),
            Span::styled(&info.author, Style::default().fg(text_secondary)),
        ])
        .render(Rect::new(x, y, w, 1), buf);
        y += 1;
    }

    // Spacer
    y += 1;

    // Description
    if y < max_y && !info.description.is_empty() {
        #[allow(clippy::as_conversions)]
        let lines = wrap_text(&info.description, w as usize);
        for line_text in &lines {
            if y >= max_y {
                break;
            }
            Line::from(Span::styled(
                line_text.as_str(),
                Style::default().fg(text_primary),
            ))
            .render(Rect::new(x, y, w, 1), buf);
            y += 1;
        }
    }

    // Spacer
    y += 1;

    // Variant label
    if y < max_y {
        let variant_label = match info.variant {
            ThemeVariant::Dark => "Dark",
            ThemeVariant::Light => "Light",
        };
        Line::from(Span::styled(
            variant_label,
            Style::default()
                .fg(text_muted)
                .add_modifier(Modifier::ITALIC),
        ))
        .render(Rect::new(x, y, w, 1), buf);
        y += 1;
    }

    // Spacer
    y += 1;

    // Color swatches — 6 semantic colors
    if y < max_y {
        let swatch_tokens = [
            tokens::ACCENT_PRIMARY,
            tokens::ACCENT_SECONDARY,
            tokens::ACCENT_TERTIARY,
            tokens::SUCCESS,
            tokens::WARNING,
            tokens::ERROR,
        ];

        let swatches: Vec<Span> = swatch_tokens
            .iter()
            .flat_map(|token| {
                [
                    Span::styled(
                        "\u{2588}\u{2588}",
                        Style::default().fg(Color::from(t.color(token))),
                    ),
                    Span::raw(" "),
                ]
            })
            .collect();

        Line::from(swatches).render(Rect::new(x, y, w, 1), buf);
        y += 1;
    }

    // Gradient bar (when gradients feature is available)
    #[cfg(feature = "gradients")]
    render_gradient_bar(t, x, y, w, max_y, buf);

    // Suppress unused variable warning when gradients feature is off
    #[cfg(not(feature = "gradients"))]
    let _ = y;
}

/// Render a gradient preview strip using the primary gradient.
#[cfg(feature = "gradients")]
fn render_gradient_bar(t: &Theme, x: u16, y: u16, w: u16, max_y: u16, buf: &mut Buffer) {
    use crate::names::gradients;

    if y >= max_y {
        return;
    }

    if let Some(gradient) = t.get_gradient(gradients::PRIMARY) {
        #[allow(clippy::as_conversions)]
        let bar_width = w.min(20) as usize;
        let spans: Vec<Span> = (0..bar_width)
            .map(|i| {
                #[allow(clippy::cast_precision_loss, clippy::as_conversions)]
                let t_val = if bar_width <= 1 {
                    0.0
                } else {
                    i as f32 / (bar_width - 1) as f32
                };
                let color = gradient.at(t_val);
                Span::styled("\u{2580}", Style::default().fg(Color::from(color)))
            })
            .collect();

        Line::from(spans).render(Rect::new(x, y, w, 1), buf);
    }
}

/// Simple word-wrap for preview descriptions.
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() > max_width {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_text_basic() {
        let result = wrap_text("hello world foo", 12);
        assert_eq!(result, vec!["hello world", "foo"]);
    }

    #[test]
    fn wrap_text_empty() {
        let result = wrap_text("", 20);
        assert!(result.is_empty());
    }

    #[test]
    fn wrap_text_single_long_word() {
        let result = wrap_text("superlongword", 5);
        assert_eq!(result, vec!["superlongword"]);
    }
}
