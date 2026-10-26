//! Tabs: a tab list that shows one panel at a time.

use hypersig::html::{El, Event, Orientation, Role, attr, button, div};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Cursor, Display, FontWeight, JustifyContent, WhiteSpace, px,
    rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Sig, Variants, if_else};
use serde::Serialize;

use crate::look;
use crate::token;

/// A tab list named `label` whose selection is the signal `selected`.
///
/// It has one tab and one panel per variant of its enum.
///
/// It follows the WAI-ARIA tabs pattern. The list has `role="tablist"`, each
/// tab `role="tab"` with `aria-selected` and `aria-controls`, and each panel
/// `role="tabpanel"` labeled by its tab. Tab enters the list at the selected
/// tab, the arrow keys, Home, and End move between tabs, and a tab is
/// selected as it gains focus, through [`El::roving_focus`]. The ids are
/// derived from the signal's path and the tab's position.
///
/// The value of `selected` is an enum with `#[derive(Segment)]`, and the
/// tabs are its variants, so the selected value always has a tab that Tab
/// can reach.
///
/// ```
/// use hypersig::html::p;
/// use hypersig::{Segment, Signals};
/// use hypersig_ui::Tabs;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
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
/// let tabs = Tabs::new("Settings", Settings::section(), |section| match section {
///     Section::Account => ("Account", p().text("Your name and email.")),
///     Section::Password => ("Password", p().text("Change your password.")),
/// });
/// let html = tabs.render().render();
/// assert!(html.contains(r#"role="tablist""#));
/// assert!(html.contains(r#"aria-controls="ui-section-panel-1""#));
/// ```
#[derive(Debug)]
pub struct Tabs<S, T> {
    label: String,
    selected: Sig<S, T>,
    /// The title and panel of each variant, in declaration order.
    entries: Vec<(String, El<S>)>,
}

impl<S, T: Variants + Serialize + PartialEq> Tabs<S, T> {
    /// A tab list named `label`, with the title and panel `tab` gives each variant.
    ///
    /// `tab` is called once per variant, in declaration order, so a `match`
    /// on the variant names a tab for each one, and the compiler rejects a
    /// `match` that leaves a variant out.
    #[must_use]
    pub fn new<L: Into<String>>(
        label: impl Into<String>,
        selected: Sig<S, T>,
        tab: impl Fn(&T) -> (L, El<S>),
    ) -> Self {
        let entries = T::VARIANTS
            .iter()
            .map(|value| {
                let (title, panel) = tab(value);
                (title.into(), panel)
            })
            .collect();
        Self {
            label: label.into(),
            selected,
            entries,
        }
    }

    /// The tab list followed by the panels, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let selected = self.selected;
        let mut list = div()
            .role(Role::TabList)
            .attr(attr::ARIA_LABEL, self.label)
            .roving_focus(Orientation::Horizontal)
            .css(list_style());
        let mut panels = Vec::with_capacity(self.entries.len());
        for (index, (value, (title, panel))) in T::VARIANTS.iter().zip(self.entries).enumerate() {
            let tab_id = crate::id::of(selected, "tab").with_key(index);
            let panel_id = crate::id::of(selected, "panel").with_key(index);
            let active = selected.equals(value);
            list = list.child(
                button()
                    .attr(attr::TYPE, "button")
                    .role(Role::Tab)
                    .id(tab_id.clone())
                    .aria_controls(&[&panel_id])
                    .bind_attr(
                        attr::ARIA_SELECTED,
                        if_else(active.clone(), "true", "false"),
                    )
                    .bind_attr(attr::TABINDEX, if_else(active.clone(), "0", "-1"))
                    .on(Event::Focus, selected.set(value))
                    .on_click(selected.set(value))
                    .css(tab_style())
                    .css_when(active_style(), active.clone())
                    .text(title),
            );
            panels.push(
                div()
                    .role(Role::TabPanel)
                    .id(panel_id)
                    .aria_labelledby(&[&tab_id])
                    .attr(attr::TABINDEX, "0")
                    .show(active)
                    .css(Style::new().margin_top(rem(0.5)).merge(look::focus_ring()))
                    .child(panel),
            );
        }
        div().child(list).children(panels)
    }
}

impl<S, T: Variants + Serialize + PartialEq> From<Tabs<S, T>> for El<S> {
    fn from(tabs: Tabs<S, T>) -> Self {
        tabs.render()
    }
}

impl<S, T: Variants + Serialize + PartialEq> From<Tabs<S, T>> for hypersig::html::Node<S> {
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
