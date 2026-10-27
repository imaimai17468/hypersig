//! Comboboxes: a text field that suggests options from a list as the visitor
//! types.

use std::fmt;

use hypersig::html::{El, Event, Key, Role, attr, div, input, label, p};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{Display, FlexDirection, OverflowY, Position, ZIndex, pct, raw, rem};
use hypersig::{Expr, NodeRef, Sig, Signals, Stmt, if_else};
use serde::{Deserialize, Serialize};

use crate::field::{field_style, label_style};
use crate::listbox::{list_style, option_style};
use crate::look::{self, icon};
use crate::token;

/// The signals a [`Combobox`] keeps.
///
/// They hold the text, whether its list is open, which option the arrow keys
/// reached, whether the pointer is on the list, and whether the text field
/// has focus.
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
    focused: bool,
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

/// A combobox: a text field with a list of options that the text filters.
///
/// It sits on a page whose signals are `S`.
///
/// Declare one as a `const` from two [`NodeRef`]s that
/// [`node_ref!`](hypersig::node_ref) declares, one for its text field and
/// one for its list, since the runtime finds both by name. Each element's id
/// is its node ref's name. It follows the WAI-ARIA combobox pattern with
/// list autocomplete. The text field has `role="combobox"`,
/// `aria-expanded`, `aria-controls`, and `aria-activedescendant`, and focus
/// stays in it while the up and down arrows move through the options shown,
/// through [`El::active_descendant`](hypersig::html::El::active_descendant).
/// Typing opens the list and shows the options that contain the text,
/// ignoring case, the down arrow opens it, Enter chooses the option the
/// arrows reached, and Escape closes the list or, when it is closed, clears
/// the text. Choosing an option puts its label in the text field and closes
/// the list, and so does leaving the field. Where the pointer is on the
/// list as focus leaves, the list closes once the pointer leaves it.
///
/// Escape is taken by the combobox while focus is in its text field, so a
/// dialog around it closes on Escape only from elsewhere. The visitor can
/// also leave text that matches no option, which the page
/// checks where it needs one of the options. The text field and the list
/// carry the node refs, so placing one combobox twice on a page makes
/// [`Page::render`](hypersig::Page::render) panic.
///
/// ```
/// use hypersig::{NodeRef, Signals};
/// use hypersig_ui::{Combobox, ComboboxState};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Form {
///     country: ComboboxState,
/// }
///
/// hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Form>);
/// hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Form>);
/// const COUNTRY: Combobox<Form> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);
///
/// # fn main() {
/// let field = COUNTRY.render("Country", Form::country(), ["France", "Japan", "Kenya"]);
/// let html = field.render();
/// let controls = format!(r#"aria-controls="{}::COUNTRY_OPTIONS""#, module_path!());
/// assert!(html.contains(r#"role="combobox""#));
/// assert!(html.contains(&controls));
/// # }
/// ```
pub struct Combobox<S> {
    field: NodeRef<S>,
    list: NodeRef<S>,
}

impl<S> Combobox<S> {
    /// A combobox whose text field `field` names and whose list `list` names.
    ///
    /// # Panics
    ///
    /// Panics if `field` and `list` are one constant. In a `const`, that is
    /// a compile error.
    #[must_use]
    pub const fn new(field: NodeRef<S>, list: NodeRef<S>) -> Self {
        assert!(
            !hypersig::_private::same_node_ref(field, list),
            "a combobox's field and list are two node refs"
        );
        Self { field, list }
    }

    /// The label, the text field, and the list of `options`, in a `<div>`.
    ///
    /// It keeps their state in `state`.
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
        let focused = state.at(ComboboxState::focused());
        let field_ref = self.field;
        let list_ref = self.list;
        let query = text.to_lowercase();

        let close: Stmt<S> = open.set(false).then(active.set(""));
        let mut none_shown: Expr<S, bool> = Expr::lit(&true);
        let mut items = Vec::new();
        for (index, title) in options.into_iter().enumerate() {
            let title: String = title.into();
            let id = self.list.id().with_key(index);
            let shown = Expr::lit(&title.to_lowercase()).contains(&query);
            none_shown = none_shown.and(!&shown);
            let reached = active.equals(id.to_string());
            items.push(
                div()
                    .role(Role::Option)
                    .id(id)
                    .bind_attr(attr::ARIA_SELECTED, reached.to_text())
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

        let field = self.field(state, &close);

        // The message sits beside the listbox rather than in it, because a
        // listbox holds only options.
        let list = div()
            .show(open)
            .on(Event::MouseEnter, pointer.set(true))
            .on(
                Event::MouseLeave,
                pointer.set(false).then(Stmt::when(!focused, close)),
            )
            .css(drop_down_style())
            .child(
                div()
                    .id(self.list.id())
                    .role(Role::ListBox)
                    .attr(attr::ARIA_LABEL, "Suggestions")
                    .node_ref(list_ref)
                    .show(!&none_shown)
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
                    .label_for(&self.field.id())
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

    /// The text field of the combobox whose state is `state`, which runs
    /// `close` to close the list.
    fn field(self, state: Sig<S, ComboboxState>, close: &Stmt<S>) -> El<S> {
        let text = state.at(ComboboxState::text());
        let open = state.at(ComboboxState::open());
        let active = state.at(ComboboxState::active());
        let pointer = state.at(ComboboxState::pointer());
        let focused = state.at(ComboboxState::focused());
        input()
            .attr(attr::TYPE, "text")
            .id(self.field.id())
            .role(Role::Combobox)
            .attr(attr::ARIA_AUTOCOMPLETE, "list")
            .aria_controls(&[&self.list.id()])
            .attr(attr::AUTOCOMPLETE, "off")
            .bind_attr(attr::ARIA_EXPANDED, if_else(open, "true", "false"))
            .bind(text)
            .node_ref(self.field)
            .active_descendant(self.list, active)
            .on_input(open.set(true).then(active.set("")))
            .on_key(Key::ArrowDown, open.set(true))
            .on_key(
                Key::Escape,
                Stmt::if_else(open, close.clone(), text.set("")),
            )
            // A click on an option blurs the field before the click lands,
            // so the list stays open while the pointer is over it, and
            // closes when the pointer leaves it if focus went elsewhere.
            .on(Event::Focus, focused.set(true))
            .on(
                Event::Blur,
                focused.set(false).then(Stmt::when(!pointer, close.clone())),
            )
            .css(field_style().height(rem(2.25)))
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
