//! Listboxes: a list of options of which one is selected.

use hypersig::html::{El, Event, Orientation, Role, attr, div};
use hypersig::style::value::{
    AlignItems, BorderStyle, Cursor, Display, JustifyContent, Opacity, px, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Sig, Variants, if_else};
use serde::Serialize;

use crate::look::{self, icon};
use crate::token;

/// A list named `label` whose selected option is the signal `selected`, one option per variant.
///
/// It follows the WAI-ARIA listbox pattern for a single selection. The list
/// has `role="listbox"` and each option `role="option"` with
/// `aria-selected`. Tab enters the list at the selected option, and the
/// arrow keys, Home, and End move between options through [`El::roving_focus`](hypersig::html::El::roving_focus),
/// selecting each as it gains focus. The ids are derived from the signal's
/// path and the option's position.
///
/// The value of `selected` is an enum with `#[derive(Segment)]`, and the
/// options are its variants, so the selected value always has an option.
///
/// ```
/// use hypersig::{Segment, Signals};
/// use hypersig_ui::Listbox;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
/// enum Plan {
///     Hobby,
///     Pro,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Billing {
///     plan: Plan,
/// }
///
/// let plans = Listbox::new("Plan", Billing::plan(), |plan| match plan {
///     Plan::Hobby => "Hobby",
///     Plan::Pro => "Pro",
/// });
/// let html = plans.render().render();
/// assert!(html.contains(r#"role="listbox""#));
/// assert!(html.contains(r#"id="ui-plan-option-1""#));
/// ```
#[derive(Debug)]
pub struct Listbox<S, T> {
    label: String,
    selected: Sig<S, T>,
    /// The title of each variant, in declaration order.
    titles: Vec<String>,
}

impl<S, T: Variants + Serialize + PartialEq> Listbox<S, T> {
    /// A list named `label`, with the title `option` gives each variant.
    ///
    /// `option` is called once per variant, in declaration order, so a
    /// `match` on the variant names a title for each one, and the compiler
    /// rejects a `match` that leaves a variant out.
    #[must_use]
    pub fn new<L: Into<String>>(
        label: impl Into<String>,
        selected: Sig<S, T>,
        option: impl Fn(&T) -> L,
    ) -> Self {
        Self {
            label: label.into(),
            selected,
            titles: T::VARIANTS
                .iter()
                .map(|value| option(value).into())
                .collect(),
        }
    }

    /// The list and its options.
    #[must_use]
    pub fn render(self) -> El<S> {
        let selected = self.selected;
        div()
            .role(Role::ListBox)
            .attr(attr::ARIA_LABEL, self.label)
            .roving_focus(Orientation::Vertical)
            .css(list_style())
            .children(T::VARIANTS.iter().zip(self.titles).enumerate().map(
                |(index, (value, title))| {
                    let chosen = selected.equals(value);
                    div()
                        .role(Role::Option)
                        .id(crate::id::of(selected, "option").with_key(index))
                        .bind_attr(
                            attr::ARIA_SELECTED,
                            if_else(chosen.clone(), "true", "false"),
                        )
                        .bind_attr(attr::TABINDEX, if_else(chosen.clone(), "0", "-1"))
                        .on(Event::Focus, selected.set(value))
                        .on_click(selected.set(value))
                        .css(option_style())
                        .text(title)
                        .child(
                            icon(look::icon::CHECK, rem(1))
                                .css(Style::new().opacity(Opacity::Number(0.0)))
                                .css_when(Style::new().opacity(Opacity::Number(1.0)), chosen),
                        )
                },
            ))
    }
}

impl<S, T: Variants + Serialize + PartialEq> From<Listbox<S, T>> for El<S> {
    fn from(listbox: Listbox<S, T>) -> Self {
        listbox.render()
    }
}

impl<S, T: Variants + Serialize + PartialEq> From<Listbox<S, T>> for hypersig::html::Node<S> {
    fn from(listbox: Listbox<S, T>) -> Self {
        listbox.render().into()
    }
}

/// The box around the options of a listbox, also used by the combobox's
/// drop-down.
pub(crate) fn list_style() -> Style {
    Style::new()
        .display(Display::Flex)
        .flex_direction(hypersig::style::value::FlexDirection::Column)
        .padding(rem(0.25))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(px(6))
        .background_color(token::popover().var())
        .color(token::popover_foreground().var())
}

/// One option: a row that is highlighted while hovered or focused.
pub(crate) fn option_style() -> Style {
    Style::new()
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::SpaceBetween)
        .gap(rem(0.5))
        .padding_block(rem(0.375))
        .padding_inline(rem(0.5))
        .border_radius(px(4))
        .cursor(Cursor::Pointer)
        .merge(look::small_text())
        .on(
            PseudoClass::Hover,
            Style::new().background_color(token::accent().var()),
        )
        .on(
            PseudoClass::Focus,
            Style::new().background_color(token::accent().var()),
        )
        .on(PseudoClass::FocusVisible, look::ring(px(-2)))
}
