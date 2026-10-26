//! Toggle buttons, alone or in a group that picks one value or several.

use hypersig::html::{El, Event, Node, Orientation, Role, attr, button as button_el, div};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Cursor, Display, FontWeight, JustifyContent, WhiteSpace, px,
    rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Sig, Writable, if_else};
use serde::Serialize;

use crate::look;
use crate::token;

/// A button that is pressed while `pressed` is true, and flips it when clicked.
///
/// It is a native `<button>` with `aria-pressed`, so a screen reader
/// announces it as a toggle button that is pressed or not pressed, and Enter
/// and Space flip it. Keep its text the same in both states, since the state
/// is announced on its own. It needs no id, so it works on a field of a list
/// item.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::toggle;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Editor {
///     bold: bool,
/// }
///
/// let html = Page::new("Editor", Editor { bold: true })
///     .child(toggle(Editor::bold()).text("Bold"))
///     .render();
/// assert!(html.contains(r#"aria-pressed="true""#));
/// ```
#[must_use]
pub fn toggle<S, R: Writable>(pressed: Sig<S, bool, R>) -> El<S> {
    pressable(pressed).on_click(pressed.toggle())
}

/// A toggle that shows only an icon, named `label` for assistive technology.
///
/// It is [`toggle`] with `aria-label` set to `label` and a square shape. Add
/// the icon with [`El::child`].
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::icon_toggle;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Editor {
///     italic: bool,
/// }
///
/// let italic = icon_toggle::<Editor, _>("Italic", Editor::italic()).text("I");
/// assert!(italic.render().contains(r#"aria-label="Italic""#));
/// ```
#[must_use]
pub fn icon_toggle<S, R: Writable>(label: impl Into<String>, pressed: Sig<S, bool, R>) -> El<S> {
    toggle(pressed)
        .attr(attr::ARIA_LABEL, label)
        .css(Style::new().width(rem(2.25)).padding_inline(px(0)))
}

/// The button of a toggle bound to `pressed`, without its click statement.
fn pressable<S, R>(pressed: Sig<S, bool, R>) -> El<S> {
    button_el()
        .attr(attr::TYPE, "button")
        .bind_attr(attr::ARIA_PRESSED, if_else(pressed, "true", "false"))
        .css(toggle_style())
        .css_when(on_style(), pressed)
}

/// A group of toggles named `label` that picks one value of `selected`.
///
/// It follows the WAI-ARIA radio group pattern, since exactly one toggle is
/// on at a time. The group has `role="radiogroup"` and each toggle
/// `role="radio"` with `aria-checked`. Tab enters the group at the toggle
/// whose value `selected` holds, and the arrow keys, Home, and End move
/// between toggles and pick the one they reach, through
/// [`El::roving_focus`]. While `selected` holds a value no toggle has, Tab
/// skips the group.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::ToggleGroup;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
/// enum Align {
///     Left,
///     Center,
///     Right,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Text {
///     align: Align,
/// }
///
/// let align = ToggleGroup::new("Alignment", Text::align())
///     .item(Align::Left, "Left")
///     .item(Align::Center, "Center")
///     .item(Align::Right, "Right");
/// let html = align.render().render();
/// assert!(html.contains(r#"role="radiogroup""#));
/// assert!(html.contains(r#"role="radio""#));
/// ```
#[derive(Debug)]
pub struct ToggleGroup<S, T> {
    label: String,
    selected: Sig<S, T>,
    /// Each toggle's value, and its button before it is bound to that value.
    items: Vec<(T, El<S>)>,
}

impl<S, T: Serialize + PartialEq> ToggleGroup<S, T> {
    /// An empty group named `label` for assistive technology.
    #[must_use]
    pub fn new(label: impl Into<String>, selected: Sig<S, T>) -> Self {
        Self {
            label: label.into(),
            selected,
            items: Vec::new(),
        }
    }

    /// Adds a toggle showing `text` that picks `value`.
    #[must_use]
    pub fn item(mut self, value: T, text: impl Into<String>) -> Self {
        self.items.push((value, button_el().text(text)));
        self
    }

    /// Adds a toggle showing `icon` that picks `value`, named `label`.
    #[must_use]
    pub fn icon_item(
        mut self,
        value: T,
        label: impl Into<String>,
        icon: impl Into<Node<S>>,
    ) -> Self {
        let item = button_el()
            .attr(attr::ARIA_LABEL, label)
            .css(Style::new().width(rem(2.25)).padding_inline(px(0)))
            .child(icon.into());
        self.items.push((value, item));
        self
    }

    /// The toggles in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let selected = self.selected;
        group(self.label, Role::RadioGroup).children(self.items.into_iter().map(|(value, item)| {
            let on = selected.equals(&value);
            item.attr(attr::TYPE, "button")
                .role(Role::Radio)
                .bind_attr(attr::ARIA_CHECKED, if_else(on.clone(), "true", "false"))
                .bind_attr(attr::TABINDEX, if_else(on.clone(), "0", "-1"))
                .on(Event::Focus, selected.set(&value))
                .on_click(selected.set(&value))
                .css(toggle_style())
                .css_when(on_style(), on)
        }))
    }
}

impl<S, T: Serialize + PartialEq> From<ToggleGroup<S, T>> for El<S> {
    fn from(group: ToggleGroup<S, T>) -> Self {
        group.render()
    }
}

impl<S, T: Serialize + PartialEq> From<ToggleGroup<S, T>> for Node<S> {
    fn from(group: ToggleGroup<S, T>) -> Self {
        group.render().into()
    }
}

/// A group of toggles named `label`, each bound to a flag of its own.
///
/// The group has `role="group"` and each toggle is a [`toggle`] with
/// `aria-pressed`. Tab enters the group at its first toggle, and the arrow
/// keys, Home, and End move between toggles without flipping them, through
/// [`El::roving_focus`]. Enter and Space flip the toggle that has focus.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::MultiToggleGroup;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Editor {
///     bold: bool,
///     italic: bool,
/// }
///
/// let style = MultiToggleGroup::new("Text style")
///     .item(Editor::bold(), "Bold")
///     .item(Editor::italic(), "Italic");
/// let html = Page::new("Editor", Editor { bold: true, italic: false })
///     .child(style)
///     .render();
/// assert!(html.contains(r#"role="group""#));
/// assert!(html.contains(r#"aria-pressed="false""#));
/// ```
#[derive(Debug)]
pub struct MultiToggleGroup<S> {
    label: String,
    items: Vec<El<S>>,
}

impl<S> MultiToggleGroup<S> {
    /// An empty group named `label` for assistive technology.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }

    /// Adds a toggle showing `text` that flips `pressed`.
    #[must_use]
    pub fn item<R: Writable>(mut self, pressed: Sig<S, bool, R>, text: impl Into<String>) -> Self {
        self.items.push(toggle(pressed).text(text));
        self
    }

    /// Adds a toggle showing `icon` that flips `pressed`, named `label`.
    #[must_use]
    pub fn icon_item<R: Writable>(
        mut self,
        pressed: Sig<S, bool, R>,
        label: impl Into<String>,
        icon: impl Into<Node<S>>,
    ) -> Self {
        self.items
            .push(icon_toggle(label, pressed).child(icon.into()));
        self
    }

    /// The toggles in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        // A `<button>` is in the tab order by itself, so every toggle but the
        // first leaves it, and the arrow keys reach them instead.
        group(self.label, Role::Group).children(
            self.items.into_iter().enumerate().map(|(index, item)| {
                item.attr(attr::TABINDEX, if index == 0 { "0" } else { "-1" })
            }),
        )
    }
}

impl<S> From<MultiToggleGroup<S>> for El<S> {
    fn from(group: MultiToggleGroup<S>) -> Self {
        group.render()
    }
}

impl<S> From<MultiToggleGroup<S>> for Node<S> {
    fn from(group: MultiToggleGroup<S>) -> Self {
        group.render().into()
    }
}

fn group<S>(label: String, role: Role) -> El<S> {
    div()
        .role(role)
        .attr(attr::ARIA_LABEL, label)
        .roving_focus(Orientation::Horizontal)
        .css(
            Style::new()
                .display(Display::InlineFlex)
                .align_items(AlignItems::Center)
                .gap(rem(0.25)),
        )
}

fn toggle_style() -> Style {
    Style::new()
        .display(Display::InlineFlex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .gap(rem(0.5))
        .height(rem(2.25))
        .min_width(rem(2.25))
        .padding_inline(rem(0.625))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::input().var())
        .border_radius(px(6))
        .background_color(Color::TRANSPARENT)
        .color(token::foreground().var())
        .font_weight(FontWeight::Number(500.0))
        .white_space(WhiteSpace::Nowrap)
        .cursor(Cursor::Pointer)
        .merge(look::small_text())
        .merge(look::transition())
        .merge(look::focus_ring())
        .on(
            PseudoClass::Hover,
            Style::new()
                .background_color(token::muted().var())
                .color(token::muted_foreground().var()),
        )
        .merge(look::disabled())
}

/// The colors a toggle takes while it is on, which hovering keeps.
fn on_style() -> Style {
    Style::new()
        .background_color(token::accent().var())
        .color(token::accent_foreground().var())
        .on(
            PseudoClass::Hover,
            Style::new()
                .background_color(token::accent().var())
                .color(token::accent_foreground().var()),
        )
}

#[cfg(test)]
mod tests {
    use hypersig::Signals;
    use serde::{Deserialize, Serialize};

    use super::{MultiToggleGroup, ToggleGroup, toggle};

    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
    enum Size {
        Small,
        Large,
    }

    #[derive(Signals, Serialize, Deserialize)]
    struct Editor {
        size: Size,
        bold: bool,
        italic: bool,
        underline: bool,
    }

    #[test]
    fn a_toggle_flips_its_flag_on_click() {
        let html = toggle(Editor::bold()).render();
        assert!(html.contains(r#"type="button""#));
        assert!(html.contains("data-hs-on-click"));
        assert!(html.contains("aria-pressed"));
    }

    #[test]
    fn a_single_group_picks_its_value_on_focus() {
        let html = ToggleGroup::new("Size", Editor::size())
            .item(Size::Small, "S")
            .item(Size::Large, "L")
            .render()
            .render();
        assert!(html.contains(r#"data-hs-roving="horizontal""#));
        assert!(html.contains("data-hs-on-focusin"));
        assert_eq!(html.matches(r#"role="radio""#).count(), 2);
    }

    #[test]
    fn a_multiple_group_keeps_only_its_first_toggle_in_the_tab_order() {
        let html = MultiToggleGroup::new("Style")
            .item(Editor::bold(), "B")
            .item(Editor::italic(), "I")
            .item(Editor::underline(), "U")
            .render()
            .render();
        assert_eq!(html.matches(r#"tabindex="0""#).count(), 1);
        assert_eq!(html.matches(r#"tabindex="-1""#).count(), 2);
    }
}
