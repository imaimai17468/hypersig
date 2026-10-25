//! Tabs: a tab list that shows one panel at a time.

use hypersig::html::{El, Event, Orientation, attr, button, div};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Cursor, Display, FontWeight, JustifyContent, WhiteSpace, px,
    rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Sig, if_else};
use serde::Serialize;

use crate::look;
use crate::token;

/// A tab list named `label` whose selection is the signal `selected`, with
/// one panel per tab.
///
/// It follows the WAI-ARIA tabs pattern. The list has `role="tablist"`, each
/// tab `role="tab"` with `aria-selected` and `aria-controls`, and each panel
/// `role="tabpanel"` labeled by its tab. Tab enters the list at the selected
/// tab, the arrow keys, Home, and End move between tabs, and a tab is
/// selected as it gains focus, through [`El::roving_focus`]. The ids are
/// derived from the signal's path and the tab's position.
///
/// ```
/// use hypersig::Signals;
/// use hypersig::html::p;
/// use hypersig_ui::Tabs;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
/// enum Section {
///     Account,
///     Password,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Settings {
///     section: Section,
/// }
///
/// let tabs = Tabs::new("Settings", Settings::section())
///     .tab(Section::Account, "Account", p().text("Your name and email."))
///     .tab(Section::Password, "Password", p().text("Change your password."));
/// let html = tabs.render().render();
/// assert!(html.contains(r#"role="tablist""#));
/// assert!(html.contains(r#"aria-controls="ui-section-panel-1""#));
/// ```
#[derive(Debug)]
pub struct Tabs<S, T> {
    label: String,
    selected: Sig<S, T>,
    entries: Vec<(T, String, El<S>)>,
}

impl<S, T: Serialize + PartialEq> Tabs<S, T> {
    /// An empty tab list named `label` for assistive technology.
    #[must_use]
    pub fn new(label: impl Into<String>, selected: Sig<S, T>) -> Self {
        Self {
            label: label.into(),
            selected,
            entries: Vec::new(),
        }
    }

    /// Adds a tab titled `title` that selects `value` and shows `panel`.
    #[must_use]
    pub fn tab(mut self, value: T, title: impl Into<String>, panel: El<S>) -> Self {
        self.entries.push((value, title.into(), panel));
        self
    }

    /// The tab list followed by the panels, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let selected = self.selected;
        let mut list = div()
            .attr(attr::ROLE, "tablist")
            .attr(attr::ARIA_LABEL, self.label)
            .roving_focus(Orientation::Horizontal)
            .css(list_style());
        let mut panels = Vec::with_capacity(self.entries.len());
        for (index, (value, title, panel)) in self.entries.into_iter().enumerate() {
            let tab_id = crate::id::of(selected, &format!("tab-{index}"));
            let panel_id = crate::id::of(selected, &format!("panel-{index}"));
            let active = selected.equals(&value);
            list = list.child(
                button()
                    .attr(attr::TYPE, "button")
                    .attr(attr::ROLE, "tab")
                    .id(tab_id.clone())
                    .attr(attr::ARIA_CONTROLS, panel_id.clone())
                    .bind_attr(
                        attr::ARIA_SELECTED,
                        if_else(active.clone(), "true", "false"),
                    )
                    .bind_attr(attr::TABINDEX, if_else(active.clone(), "0", "-1"))
                    .on(Event::Focus, selected.set(&value))
                    .on_click(selected.set(&value))
                    .css(tab_style())
                    .css_when(active_style(), active.clone())
                    .text(title),
            );
            panels.push(
                div()
                    .attr(attr::ROLE, "tabpanel")
                    .id(panel_id)
                    .attr(attr::ARIA_LABELLEDBY, tab_id)
                    .attr(attr::TABINDEX, "0")
                    .show(active)
                    .css(Style::new().margin_top(rem(0.5)).merge(look::focus_ring()))
                    .child(panel),
            );
        }
        div().child(list).children(panels)
    }
}

impl<S, T: Serialize + PartialEq> From<Tabs<S, T>> for El<S> {
    fn from(tabs: Tabs<S, T>) -> Self {
        tabs.render()
    }
}

impl<S, T: Serialize + PartialEq> From<Tabs<S, T>> for hypersig::html::Node<S> {
    fn from(tabs: Tabs<S, T>) -> Self {
        tabs.render().into()
    }
}

fn list_style() -> Style {
    Style::new()
        .display(Display::InlineFlex)
        .align_items(AlignItems::Center)
        .height(rem(2.25))
        .padding(px(3))
        .border_radius(token::radius().var())
        .background_color(token::muted().var())
        .color(token::muted_foreground().var())
}

fn tab_style() -> Style {
    Style::new()
        .display(Display::InlineFlex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .height(hypersig::style::value::pct(100))
        .padding_inline(rem(0.75))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(Color::TRANSPARENT)
        .border_radius(px(6))
        .background_color(Color::TRANSPARENT)
        .color(hypersig::style::value::Color::CURRENT_COLOR)
        .font_weight(FontWeight::Number(500.0))
        .white_space(WhiteSpace::Nowrap)
        .cursor(Cursor::Pointer)
        .merge(look::small_text())
        .merge(look::transition())
        .merge(look::focus_ring())
        .on(
            PseudoClass::Hover,
            Style::new().color(token::foreground().var()),
        )
        .merge(look::disabled())
}

fn active_style() -> Style {
    Style::new()
        .background_color(token::background().var())
        .color(token::foreground().var())
        .box_shadow(hypersig::style::value::raw("0 1px 2px 0 rgb(0 0 0 / 0.05)"))
}
