//! # Opaline Theme Showcase
//!
//! Interactive TUI demo for the Opaline theme engine. Browse all 20 builtin
//! themes, see styles rendered in real-time, and explore gradient capabilities.
//!
//! ```bash
//! cargo run --example theme-showcase
//! ```

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use opaline::{
    gradient_bar, list_available_themes, load_by_name, Theme, ThemeInfo,
    ThemeRatatuiExt,
};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

// ── App State ────────────────────────────────────────────────────────────────

struct App {
    themes: Vec<ThemeInfo>,
    list_state: ListState,
    theme: Theme,
    focus: Panel,
    should_quit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Panel {
    Themes,
    Styles,
    Gradients,
}

impl App {
    fn new() -> Self {
        let themes = list_available_themes();
        let theme = Theme::default();
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            themes,
            list_state,
            theme,
            focus: Panel::Themes,
            should_quit: false,
        }
    }

    fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    fn next_theme(&mut self) {
        let i = (self.selected_index() + 1) % self.themes.len();
        self.list_state.select(Some(i));
        self.load_selected();
    }

    fn prev_theme(&mut self) {
        let i = if self.selected_index() == 0 {
            self.themes.len() - 1
        } else {
            self.selected_index() - 1
        };
        self.list_state.select(Some(i));
        self.load_selected();
    }

    fn load_selected(&mut self) {
        if let Some(info) = self.themes.get(self.selected_index()) {
            if let Some(loaded) = load_by_name(&info.name) {
                self.theme = loaded;
            }
        }
    }

    fn next_panel(&mut self) {
        self.focus = match self.focus {
            Panel::Themes => Panel::Styles,
            Panel::Styles => Panel::Gradients,
            Panel::Gradients => Panel::Themes,
        };
    }
}

// ── Entrypoint ───────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                handle_key(&mut app, key.code);
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Tab | KeyCode::BackTab => app.next_panel(),
        KeyCode::Up | KeyCode::Char('k') => app.prev_theme(),
        KeyCode::Down | KeyCode::Char('j') => app.next_theme(),
        KeyCode::Enter => app.load_selected(),
        _ => {}
    }
}

// ── UI Layout ────────────────────────────────────────────────────────────────

fn ui(frame: &mut Frame, app: &mut App) {
    let bg = app.theme.ratatui_color("bg.base");
    frame.render_widget(
        Block::default().style(Style::default().bg(bg)),
        frame.area(),
    );

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let [left, center, right] = Layout::horizontal([
        Constraint::Min(24),
        Constraint::Percentage(42),
        Constraint::Percentage(30),
    ])
    .areas(body);

    render_header(frame, app, header);
    render_theme_list(frame, app, left);
    render_styles(frame, app, center);
    render_gradients(frame, app, right);
    render_footer(frame, app, footer);
}

// ── Panel Renderers ──────────────────────────────────────────────────────────

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered()
        .style(panel_style(app))
        .border_style(app.theme.ratatui_style("focused_border"));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let title = app
        .theme
        .gradient_styled_line("aurora", "\u{2726} Opaline Theme Showcase");
    frame.render_widget(
        Paragraph::new(title).alignment(Alignment::Center),
        inner,
    );
}

fn render_theme_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let border_style = panel_border(app, Panel::Themes);

    let base = panel_style(app);

    let items: Vec<ListItem> = app
        .themes
        .iter()
        .map(|info| {
            let marker = if info.variant == opaline::ThemeVariant::Dark {
                "\u{25cf}"
            } else {
                "\u{25cb}"
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {marker} "),
                    app.theme.ratatui_style("muted"),
                ),
                Span::styled(
                    info.display_name.clone(),
                    Style::default().fg(app.theme.ratatui_color("text.secondary")),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(Line::styled(
                    " Themes ",
                    app.theme.ratatui_style("keyword"),
                ))
                .style(base)
                .border_style(border_style),
        )
        .highlight_style(app.theme.ratatui_style("active_selected"))
        .highlight_symbol("\u{25b8} ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_styles(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = panel_border(app, Panel::Styles);
    let block = Block::bordered()
        .title(Line::styled(
            " Styles & Tokens ",
            app.theme.ratatui_style("keyword"),
        ))
        .style(panel_style(app))
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [styles_area, tokens_area] = Layout::vertical([
        Constraint::Min(16),
        Constraint::Fill(1),
    ])
    .areas(inner);

    render_style_samples(frame, app, styles_area);
    render_token_swatches(frame, app, tokens_area);
}

fn render_style_samples(frame: &mut Frame, app: &App, area: Rect) {
    let samples: &[(&str, &str)] = &[
        ("keyword", "fn main()"),
        ("file_path", "src/lib.rs"),
        ("commit_hash", "a1b2c3d"),
        ("success_style", "\u{2713} Tests passed"),
        ("error_style", "\u{2717} Build failed"),
        ("warning_style", "\u{26a0} Deprecated"),
        ("info_style", "\u{2139} 20 themes loaded"),
        ("dimmed", "subtle hint text"),
        ("muted", "secondary content"),
        ("inline_code", " let x = 42 "),
        ("diff_added", "+ added line"),
        ("diff_removed", "- removed line"),
        ("git_staged", "\u{25cf} staged"),
        ("git_modified", "\u{25cf} modified"),
        ("author", "hyperb1iss"),
        ("timestamp", "2024-01-15 09:30"),
    ];

    let label_style = Style::default().fg(app.theme.ratatui_color("text.secondary"));

    let lines: Vec<Line> = samples
        .iter()
        .map(|(name, sample)| {
            Line::from(vec![
                Span::styled(format!(" {name:<16} "), label_style),
                Span::styled(
                    (*sample).to_string(),
                    app.theme.ratatui_style(name),
                ),
            ])
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_token_swatches(frame: &mut Frame, app: &App, area: Rect) {
    let tokens = [
        "text.primary",
        "text.secondary",
        "text.muted",
        "accent.primary",
        "accent.secondary",
        "success",
        "error",
        "warning",
        "info",
    ];

    let label_style = Style::default().fg(app.theme.ratatui_color("text.secondary"));

    let mut lines: Vec<Line> = vec![Line::styled(
        " \u{2500}\u{2500} Color Tokens \u{2500}\u{2500}",
        app.theme.ratatui_style("muted"),
    )];

    for name in &tokens {
        let color = app.theme.ratatui_color(name);
        lines.push(Line::from(vec![
            Span::styled(format!(" {name:<18} "), label_style),
            Span::styled(
                "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
                Style::default().fg(color),
            ),
        ]));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_gradients(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = panel_border(app, Panel::Gradients);
    let block = Block::bordered()
        .title(Line::styled(
            " Gradients ",
            app.theme.ratatui_style("keyword"),
        ))
        .style(panel_style(app))
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [bars_area, meta_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
    ])
    .areas(inner);

    render_gradient_bars(frame, app, bars_area);
    render_theme_meta(frame, app, meta_area);
}

fn render_gradient_bars(frame: &mut Frame, app: &App, area: Rect) {
    let names = [
        "primary",
        "warm",
        "aurora",
        "success_gradient",
        "error_gradient",
    ];
    let bar_width = usize::from(area.width.saturating_sub(4));
    let mut lines: Vec<Line> = Vec::new();

    let label_style = Style::default().fg(app.theme.ratatui_color("text.secondary"));

    for name in &names {
        lines.push(Line::styled(format!("  {name}"), label_style));

        if let Some(gradient) = app.theme.get_gradient(name) {
            let bar = gradient_bar(bar_width, '\u{2588}', gradient);
            lines.push(Line::from(
                std::iter::once(Span::raw("  "))
                    .chain(bar.spans)
                    .collect::<Vec<_>>(),
            ));
        }

        lines.push(Line::raw(""));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_theme_meta(frame: &mut Frame, app: &App, area: Rect) {
    let meta = &app.theme.meta;
    let variant_str = if app.theme.is_dark() { "Dark" } else { "Light" };
    let author = meta.author.as_deref().unwrap_or("\u{2014}");
    let desc = meta.description.as_deref().unwrap_or("");

    let label_style = Style::default().fg(app.theme.ratatui_color("text.muted"));

    let lines = vec![
        Line::from(vec![
            Span::styled("  Theme   ", label_style),
            app.theme.ratatui_span("keyword", meta.name.clone()),
        ]),
        Line::from(vec![
            Span::styled("  Author  ", label_style),
            app.theme.ratatui_span("file_path", author.to_string()),
        ]),
        Line::from(vec![
            Span::styled("  Variant ", label_style),
            Span::styled(variant_str, app.theme.ratatui_style("info_style")),
        ]),
        Line::styled(
            format!("  {desc}"),
            app.theme.ratatui_style("muted"),
        ),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let sep = Style::default().fg(app.theme.ratatui_color("text.secondary"));
    let key = app.theme.ratatui_style("keyword");

    let hints = Line::from(vec![
        Span::styled(" \u{2191}\u{2193}/jk", key),
        Span::styled(" Navigate  ", sep),
        Span::styled("Tab", key),
        Span::styled(" Focus  ", sep),
        Span::styled("Enter", key),
        Span::styled(" Load  ", sep),
        Span::styled("q", key),
        Span::styled(" Quit", sep),
    ]);

    let footer_bg = Style::default()
        .fg(app.theme.ratatui_color("text.secondary"))
        .bg(app.theme.ratatui_color("bg.base"));

    frame.render_widget(Paragraph::new(hints).style(footer_bg), area);
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Base style for panel content: theme fg on theme bg.
/// Ensures every text element has explicit colors — never inherits from the
/// terminal emulator, which would break contrast on light themes.
fn panel_style(app: &App) -> Style {
    Style::default()
        .fg(app.theme.ratatui_color("text.primary"))
        .bg(app.theme.ratatui_color("bg.panel"))
}

fn panel_border(app: &App, panel: Panel) -> Style {
    if app.focus == panel {
        app.theme.ratatui_style("focused_border")
    } else {
        app.theme.ratatui_style("unfocused_border")
    }
}
