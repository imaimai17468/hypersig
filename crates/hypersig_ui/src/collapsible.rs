//! Collapsibles: a button that shows and hides one region.

use hypersig::html::{El, Id, attr, div};
use hypersig::{Sig, if_else};

use crate::button::{ButtonSize, ButtonVariant, button};

/// A region shown while `open` is true, and the button that flips it.
///
/// It follows the WAI-ARIA disclosure pattern. The button has
/// `aria-expanded` bound to `open` and `aria-controls` pointing at the
/// region, so a screen reader announces it as expanded or collapsed, and
/// Enter and Space flip it. The region's id is derived from the signal's
/// path. The button and the region are separate elements, so the button can
/// sit in a heading or a toolbar away from the region.
///
/// Use [`Accordion`](crate::Accordion) for a stack of sections the visitor
/// opens without the server or other signals reading which one is open.
///
/// ```
/// use hypersig::html::p;
/// use hypersig::{Page, Signals};
/// use hypersig_ui::{ButtonSize, ButtonVariant, Collapsible};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Order {
///     details: bool,
/// }
///
/// let details = Collapsible::new(Order::details());
/// let html = Page::new("Order", Order { details: false })
///     .child(details.trigger(ButtonVariant::Ghost, ButtonSize::Sm).text("Details"))
///     .child(details.content().child(p().text("Shipped on Monday.")))
///     .render();
/// assert!(html.contains(r#"aria-expanded="false""#));
/// assert!(html.contains(r#"aria-controls="ui-details-content""#));
/// ```
pub struct Collapsible<S> {
    open: Sig<S, bool>,
}

impl<S> Collapsible<S> {
    /// A collapsible whose region is shown while `open` is true.
    #[must_use]
    pub fn new(open: Sig<S, bool>) -> Self {
        Self { open }
    }

    fn content_id(self) -> Id {
        crate::id::of(self.open, "content")
    }

    /// The button that shows and hides the region.
    #[must_use]
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S> {
        let open = self.open;
        button(variant, size)
            .aria_controls(&[&self.content_id()])
            .bind_attr(attr::ARIA_EXPANDED, if_else(open, "true", "false"))
            .on_click(open.toggle())
    }

    /// The region, empty, for the content added with [`El::child`].
    #[must_use]
    pub fn content(self) -> El<S> {
        div().id(self.content_id()).show(self.open)
    }
}

impl<S> Clone for Collapsible<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Collapsible<S> {}

impl<S> std::fmt::Debug for Collapsible<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Collapsible").field(&self.open).finish()
    }
}

#[cfg(test)]
mod tests {
    use hypersig::{Page, Signals};
    use serde::{Deserialize, Serialize};

    use super::Collapsible;
    use crate::{ButtonSize, ButtonVariant};

    #[derive(Signals, Serialize, Deserialize)]
    struct Order {
        details: bool,
    }

    #[test]
    fn the_region_is_hidden_while_closed_and_the_button_flips_it() {
        let details = Collapsible::new(Order::details());
        let html = Page::new("Order", Order { details: false })
            .child(details.trigger(ButtonVariant::Ghost, ButtonSize::Sm))
            .child(details.content())
            .render();
        assert!(html.contains(r#"id="ui-details-content""#));
        assert!(html.contains("data-hs-on-click"));
        assert!(html.contains(r#"aria-expanded="false""#));
    }
}
