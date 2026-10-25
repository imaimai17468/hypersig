//! Comboboxes: a text field that suggests options from a list as the visitor
//! types.

use std::fmt;
use std::marker::PhantomData;

use hypersig::html::{El, Event, Key, attr, div, input, label, p};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{Display, FlexDirection, OverflowY, Position, ZIndex, pct, raw, rem};
use hypersig::{Expr, NodeRef, Sig, Signals, Stmt, if_else};
use serde::{Deserialize, Serialize};

use crate::field::{field_style, label_style};
use crate::listbox::{list_style, option_style};
use crate::look::{self, icon};
use crate::token;

/// The signals a [`Combobox`] keeps: the text, whether its list is open,
/// which option the arrow keys reached, and whether the pointer is on the
/// list.
///
/// Put one in the page's signals for each combobox and read the chosen text
/// from [`ComboboxState::text`].
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::ComboboxState;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Form {
///     country: ComboboxState,
/// }
///
/// let form = Form { country: ComboboxState::new("Japan") };
/// assert_eq!(form.country.text, "Japan");
/// ```
#[derive(Signals, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ComboboxState {
    /// What the text field holds: what the visitor typed, or the option they
    /// chose.
    pub text: String,
    open: bool,
    active: String,
    pointer: bool,
}

impl ComboboxState {
    /// A closed combobox whose text field holds `text`.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

/// A combobox on a page whose signals are `S`: a text field with a list of
/// options that the text filters.
///
/// Declare one as a `const` naming its text field and its list, since the
/// runtime finds both by name. It follows the WAI-ARIA combobox pattern with
/// list autocomplete. The text field has `role="combobox"`,
/// `aria-expanded`, `aria-controls`, and `aria-activedescendant`, and focus
/// stays in it while the up and down arrows move through the options shown,
/// through [`El::active_descendant`](hypersig::html::El::active_descendant).
/// Typing opens the list and shows the options that contain the text,
/// ignoring case, the down arrow opens it, Enter chooses the option the
/// arrows reached, and Escape closes the list or, when it is closed, clears
/// the text. Choosing an option puts its label in the text field and closes
/// the list, and so does leaving the field, unless the pointer is on the
/// list.
///
/// Escape is taken by the combobox while focus is in its text field, so a
/// dialog around it closes on Escape only from elsewhere. The visitor can
/// also leave text that matches no option, which the page
/// checks where it needs one of the options. The text field and the list
/// carry node refs with their names, so two comboboxes with one name on a page
/// make [`Page::render`](hypersig::Page::render) panic.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::{Combobox, ComboboxState};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Form {
///     country: ComboboxState,
/// }
///
/// const COUNTRY: Combobox<Form> = Combobox::new("country", "country-options");
///
/// let field = COUNTRY.render("Country", Form::country(), ["France", "Japan", "Kenya"]);
/// let html = field.render();
/// assert!(html.contains(r#"role="combobox""#));
/// assert!(html.contains(r#"aria-controls="country-options""#));
/// ```
pub struct Combobox<S> {
    field: &'static str,
    list: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> Combobox<S> {
    /// A combobox whose text field has the id `field` and whose list has the
    /// id `list`.
    ///
    /// # Panics
    ///
    /// Panics if either name is empty or holds a character other than an
    /// ASCII letter, digit, `-`, or `_`, or if the two are equal. In a
    /// `const`, that is a compile error.
    #[must_use]
    pub const fn new(field: &'static str, list: &'static str) -> Self {
        crate::id::check_name(field);
        crate::id::check_name(list);
        assert!(
            !crate::id::same(field, list),
            "a combobox's field and list have different names"
        );
        Self {
            field,
            list,
            marker: PhantomData,
        }
    }

    /// The label, the text field, and the list of `options`, in a `<div>`,
    /// keeping their state in `state`.
    #[must_use]
    pub fn render<L: Into<String>>(
        self,
        label_text: impl Into<String>,
        state: Sig<S, ComboboxState>,
        options: impl IntoIterator<Item = L>,
    ) -> El<S> {
        let text = state.at(ComboboxState::text());
        let open = state.at(ComboboxState::open());
        let active = state.at(ComboboxState::active());
        let pointer = state.at(ComboboxState::pointer());
        let field_ref = NodeRef::new(self.field);
        let list_ref = NodeRef::new(self.list);
        let query = text.to_lowercase();

        let close: Stmt<S> = open.set(false).then(active.set(""));
        let mut none_shown: Expr<S, bool> = Expr::lit(&true);
        let mut items = Vec::new();
        for (index, title) in options.into_iter().enumerate() {
            let title: String = title.into();
            let id = format!("{}-{index}", self.list);
            let shown = Expr::lit(&title.to_lowercase()).contains(query.clone());
            none_shown = none_shown.and(!shown.clone());
            let reached = active.equals(id.clone());
            items.push(
                div()
                    .attr(attr::ROLE, "option")
                    .id(id)
                    .bind_attr(attr::ARIA_SELECTED, reached.clone().to_text())
                    .show(shown)
                    .on_click(
                        text.set(title.clone())
                            .then(close.clone())
                            .then(pointer.set(false))
                            .then(field_ref.focus()),
                    )
                    .css(option_style())
                    .css_when(
                        Style::new().background_color(token::accent().var()),
                        reached,
                    )
                    .text(title.clone())
                    .child(icon(look::icon::CHECK, rem(1)).show(text.equals(title))),
            );
        }

        let field = input()
            .attr(attr::TYPE, "text")
            .id(self.field)
            .attr(attr::ROLE, "combobox")
            .attr(attr::ARIA_AUTOCOMPLETE, "list")
            .attr(attr::ARIA_CONTROLS, self.list)
            .attr(attr::AUTOCOMPLETE, "off")
            .bind_attr(attr::ARIA_EXPANDED, if_else(open, "true", "false"))
            .bind_attr(attr::ARIA_ACTIVEDESCENDANT, active)
            .bind(text)
            .node_ref(field_ref)
            .active_descendant(list_ref, active)
            .on_input(open.set(true).then(active.set("")))
            .on_key(Key::ArrowDown, open.set(true))
            .on_key(
                Key::Escape,
                Stmt::if_else(open, close.clone(), text.set("")),
            )
            .on(Event::Blur, Stmt::when(!pointer, close))
            .css(field_style().height(rem(2.25)));

        // The message sits beside the listbox rather than in it, because a
        // listbox holds only options.
        let list = div()
            .show(open)
            .on(Event::MouseEnter, pointer.set(true))
            .on(Event::MouseLeave, pointer.set(false))
            .css(drop_down_style())
            .child(
                div()
                    .id(self.list)
                    .attr(attr::ROLE, "listbox")
                    .attr(attr::ARIA_LABEL, "Suggestions")
                    .node_ref(list_ref)
                    .show(!none_shown.clone())
                    .css(
                        Style::new()
                            .display(Display::Flex)
                            .flex_direction(FlexDirection::Column),
                    )
                    .children(items),
            )
            .child(
                p().css(
                    look::small_text()
                        .padding_block(rem(0.375))
                        .padding_inline(rem(0.5))
                        .color(token::muted_foreground().var()),
                )
                .show(none_shown)
                .text("No matches"),
            );

        div()
            .css(stack(rem(0.5)))
            .child(
                label()
                    .attr(attr::FOR, self.field)
                    .css(label_style())
                    .text(label_text),
            )
            .child(
                div()
                    .css(Style::new().position(Position::Relative))
                    .child(field)
                    .child(list),
            )
    }
}

impl<S> Clone for Combobox<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Combobox<S> {}

impl<S> fmt::Debug for Combobox<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Combobox")
            .field("field", &self.field)
            .field("list", &self.list)
            .finish()
    }
}

fn drop_down_style() -> Style {
    list_style()
        .position(Position::Absolute)
        .top(raw("calc(100% + 0.25rem)"))
        .left(rem(0))
        .width(pct(100))
        .max_height(rem(15))
        .overflow_y(OverflowY::Auto)
        .z_index(ZIndex::Integer(50))
        .box_shadow(raw(
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
        ))
}
