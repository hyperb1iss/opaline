//! egui adapter — apply Opaline themes to immediate-mode GUIs.
//!
//! Provides `From` conversions for [`Color32`](egui::Color32) and a
//! [`to_egui_visuals`] function that maps theme tokens onto egui's
//! [`Visuals`](egui::Visuals), starting from the appropriate dark/light
//! base and overriding all color properties.
//!
//! ```rust,ignore
//! let theme = opaline::Theme::default();
//! let visuals = opaline::adapters::egui::to_egui_visuals(&theme);
//! ctx.set_visuals(visuals);
//! ```

use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{Color32, Stroke, Visuals};

use crate::color::OpalineColor;
use crate::theme::Theme;

// ═══════════════════════════════════════════════════════════════════════════════
// Color conversion
// ═══════════════════════════════════════════════════════════════════════════════

impl From<OpalineColor> for Color32 {
    fn from(c: OpalineColor) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}

impl From<&OpalineColor> for Color32 {
    fn from(c: &OpalineColor) -> Self {
        Color32::from_rgb(c.r, c.g, c.b)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Theme → Visuals
// ═══════════════════════════════════════════════════════════════════════════════

/// Convert an Opaline [`Theme`] to egui [`Visuals`].
///
/// Starts from [`Visuals::dark()`] or [`Visuals::light()`] based on the
/// theme variant, then overrides all color-related fields from theme tokens.
/// Non-color fields (corner radii, shadows, expansion) retain their defaults.
pub fn to_egui_visuals(theme: &Theme) -> Visuals {
    let mut v = if theme.is_dark() {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    let bg_base: Color32 = theme.color("bg.base").into();
    let bg_panel: Color32 = theme.color("bg.panel").into();
    let bg_highlight: Color32 = theme.color("bg.highlight").into();
    let bg_code: Color32 = theme.color("bg.code").into();
    let bg_selection: Color32 = theme.color("bg.selection").into();

    let text_primary: Color32 = theme.color("text.primary").into();
    let text_secondary: Color32 = theme.color("text.secondary").into();
    let text_muted: Color32 = theme.color("text.muted").into();

    let accent: Color32 = theme.color("accent.primary").into();
    let accent_secondary: Color32 = theme.color("accent.secondary").into();

    let border_focused: Color32 = theme.color("border.focused").into();
    let border_unfocused: Color32 = theme.color("border.unfocused").into();

    let extreme_bg = if theme.is_dark() {
        theme.color("bg.base").darken(0.5)
    } else {
        theme.color("bg.base").lighten(0.5)
    };

    // ── Global colors ────────────────────────────────────────────────────

    v.dark_mode = theme.is_dark();
    v.override_text_color = None;
    v.panel_fill = bg_base;
    v.window_fill = bg_panel;
    v.faint_bg_color = bg_highlight;
    v.extreme_bg_color = extreme_bg.into();
    v.code_bg_color = bg_code;
    v.hyperlink_color = accent;
    v.warn_fg_color = theme.color("warning").into();
    v.error_fg_color = theme.color("error").into();
    v.window_stroke = Stroke::new(1.0, border_unfocused);

    // ── Selection ────────────────────────────────────────────────────────

    v.selection = Selection {
        bg_fill: bg_selection,
        stroke: Stroke::new(1.0, accent),
    };

    // ── Widgets ──────────────────────────────────────────────────────────

    v.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: bg_base,
            weak_bg_fill: bg_base,
            bg_stroke: Stroke::new(1.0, border_unfocused),
            fg_stroke: Stroke::new(1.0, text_muted),
            ..v.widgets.noninteractive
        },
        inactive: WidgetVisuals {
            bg_fill: bg_panel,
            weak_bg_fill: bg_panel,
            bg_stroke: Stroke::new(1.0, border_unfocused),
            fg_stroke: Stroke::new(1.0, text_secondary),
            ..v.widgets.inactive
        },
        hovered: WidgetVisuals {
            bg_fill: bg_highlight,
            weak_bg_fill: bg_highlight,
            bg_stroke: Stroke::new(1.0, border_focused),
            fg_stroke: Stroke::new(1.5, accent),
            ..v.widgets.hovered
        },
        active: WidgetVisuals {
            bg_fill: bg_selection,
            weak_bg_fill: bg_selection,
            bg_stroke: Stroke::new(1.0, accent),
            fg_stroke: Stroke::new(2.0, accent),
            ..v.widgets.active
        },
        open: WidgetVisuals {
            bg_fill: bg_highlight,
            weak_bg_fill: bg_highlight,
            bg_stroke: Stroke::new(1.0, accent_secondary),
            fg_stroke: Stroke::new(1.0, text_primary),
            ..v.widgets.open
        },
    };

    v
}
