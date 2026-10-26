//! Accordions: a stack of sections that each open and close.

use hypersig::html::{El, Id, Node, attr, details, div, summary};
use hypersig::style::value::{
    AlignItems, BorderBottomStyle, Cursor, Display, FontWeight, JustifyContent, ListStyleType, px,
    raw, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};

use crate::look::{self, icon};
use crate::token;

/// A stack of sections, each a native `<details>` whose `<summary>` opens
/// and closes it.
///
/// The browser handles the keyboard and announces each summary as a button
/// that is expanded or collapsed, and the find-in-page search opens a closed
/// section that holds a match. An exclusive accordion gives its sections one
/// `name`, so the browser closes the open one when another opens.
///
/// ```
/// use hypersig::Signals;
/// use hypersig::html::{Id, p};
/// use hypersig_ui::Accordion;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Faq {}
///
/// const FAQ: Id = Id::new("faq");
///
/// let faq = Accordion::<Faq>::exclusive(FAQ)
///     .item("Is it accessible?", p().text("It follows the WAI-ARIA patterns."))
///     .item("Is it styled?", p().text("It looks like shadcn/ui."));
/// let html = faq.render().render();
/// assert!(html.starts_with(r#"<div id="faq">"#));
/// assert_eq!(html.matches(r#" name="faq""#).count(), 2);
/// ```
#[derive(Debug)]
pub struct Accordion<S> {
    group: Option<Id>,
    items: Vec<(String, Node<S>)>,
}

impl<S> Accordion<S> {
    /// An accordion whose sections open and close independently.
    #[must_use]
    pub fn new() -> Self {
        Self {
            group: None,
            items: Vec::new(),
        }
    }

    /// An accordion that keeps at most one section open.
    ///
    /// Its `<div>` carries the id `group`.
    ///
    /// The sections share `group` as their `name`, and sections with one
    /// `name` close each other wherever they are on the page. The `<div>`
    /// carries `group` as its id, so two accordions given one [`Id::new`]
    /// are two elements with one id, which
    /// [`Page::render`](hypersig::Page::render) panics on. An id built with
    /// [`Id::with_key`] or [`Id::from_text`] is not checked, and two
    /// accordions given one such id close each other's sections.
    #[must_use]
    pub fn exclusive(group: Id) -> Self {
        Self {
            group: Some(group),
            items: Vec::new(),
        }
    }

    /// Adds a section titled `title` whose content is `content`.
    #[must_use]
    pub fn item(mut self, title: impl Into<String>, content: impl Into<Node<S>>) -> Self {
        self.items.push((title.into(), content.into()));
        self
    }

    /// The sections, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let name = self.group.as_ref().map(|group| group.as_str().to_owned());
        let list = match self.group {
            Some(group) => div().id(group),
            None => div(),
        };
        list.children(self.items.into_iter().map(|(title, content)| {
            let section = details().css(
                Style::new()
                    .border_bottom_width(px(1))
                    .border_bottom_style(BorderBottomStyle::Solid)
                    .border_bottom_color(token::border().var())
                    .group(),
            );
            let section = match &name {
                Some(name) => section.attr(attr::NAME, name.clone()),
                None => section,
            };
            section
                .child(
                    summary().css(summary_style()).text(title).child(
                        icon(look::icon::CHEVRON_DOWN, rem(1)).css(
                            Style::new()
                                .color(token::muted_foreground().var())
                                .merge(look::transition())
                                .on(
                                    Condition::Group(PseudoClass::Open),
                                    Style::new().rotate(raw("180deg")),
                                ),
                        ),
                    ),
                )
                .child(
                    div()
                        .css(look::small_text().padding_bottom(rem(1)))
                        .child(content),
                )
        }))
    }
}

impl<S> Default for Accordion<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<Accordion<S>> for El<S> {
    fn from(accordion: Accordion<S>) -> Self {
        accordion.render()
    }
}

impl<S> From<Accordion<S>> for Node<S> {
    fn from(accordion: Accordion<S>) -> Self {
        accordion.render().into()
    }
}

fn summary_style() -> Style {
    // A summary that is not `display: list-item` draws no disclosure marker.
    Style::new()
        .display(Display::Flex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::SpaceBetween)
        .gap(rem(1))
        .padding_block(rem(1))
        .list_style_type(ListStyleType::None)
        .font_weight(FontWeight::Number(500.0))
        .cursor(Cursor::Pointer)
        .border_radius(px(6))
        .merge(look::small_text())
        .merge(look::focus_ring())
        .on(
            PseudoClass::Hover,
            Style::new()
                .text_decoration_line(hypersig::style::value::TextDecorationLine::Underline),
        )
}

#[cfg(test)]
mod tests {
    use hypersig::html::{Id, div, p};

    use super::Accordion;

    const FAQ: Id = Id::new("faq");

    #[test]
    #[should_panic(expected = "two elements carry the id `faq`")]
    fn two_exclusive_accordions_of_one_group_panic_when_rendered() {
        let faq = || Accordion::<()>::exclusive(FAQ).item("Why?", p().text("Because."));
        let _ = div::<()>().child(faq()).child(faq()).render();
    }

    #[test]
    fn each_section_of_an_exclusive_accordion_carries_its_group_as_name() {
        let html = Accordion::<()>::exclusive(FAQ)
            .item("One", p())
            .item("Two", p())
            .render()
            .render();
        assert_eq!(html.matches(r#" name="faq""#).count(), 2);
    }
}
