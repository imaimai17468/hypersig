//! Avatars: a round picture of a person, with their initials in its place.

use hypersig::IntoHref;
use hypersig::html::{El, Node, Role, attr, img, span};
use hypersig::style::value::{
    AlignItems, Display, FontWeight, JustifyContent, ObjectFit, Overflow, Position, VerticalAlign,
    px, raw, rem,
};
use hypersig::style::{PseudoElement, Style};

use crate::token;

/// A round picture of the person `name`, or `initials` where there is none.
///
/// The avatar is a `role="img"` named `name`, so a screen reader reads the
/// name once and nothing inside it. Without [`Avatar::picture`] it shows
/// `initials`. With one, it shows the picture, and where the picture fails
/// to load, the `::after` of the broken `<img>` draws `initials` over it.
/// Chrome draws that `::after`. A browser that draws no `::after` on a
/// broken image shows its own marker for one there instead.
///
/// ```
/// use hypersig::{SafeUrl, Signals};
/// use hypersig_ui::Avatar;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Team {}
///
/// const ADA: SafeUrl = SafeUrl::new("https://example.com/ada.png");
///
/// let ada = Avatar::<Team>::new("Ada Lovelace", "AL").picture(ADA);
/// let html = ada.render().render();
/// assert!(html.contains(r#"aria-label="Ada Lovelace""#));
/// assert!(html.contains(r#"alt="AL""#));
/// ```
#[derive(Debug)]
pub struct Avatar<S> {
    name: String,
    initials: String,
    picture: Option<El<S>>,
}

impl<S> Avatar<S> {
    /// An avatar of `name` that shows `initials`.
    #[must_use]
    pub fn new(name: impl Into<String>, initials: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            initials: initials.into(),
            picture: None,
        }
    }

    /// Shows the picture at `url` in place of the initials.
    #[must_use]
    pub fn picture(mut self, url: impl IntoHref<S>) -> Self {
        self.picture = Some(img().src(url));
        self
    }

    /// The avatar, as a `<span>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let avatar = span()
            .role(Role::Img)
            .attr(attr::ARIA_LABEL, self.name)
            .css(avatar_style());
        match self.picture {
            Some(picture) => {
                avatar.child(picture.attr(attr::ALT, self.initials).css(picture_style()))
            }
            None => avatar.text(self.initials),
        }
    }
}

impl<S> From<Avatar<S>> for El<S> {
    fn from(avatar: Avatar<S>) -> Self {
        avatar.render()
    }
}

impl<S> From<Avatar<S>> for Node<S> {
    fn from(avatar: Avatar<S>) -> Self {
        avatar.render().into()
    }
}

fn avatar_style() -> Style {
    Style::new()
        .position(Position::Relative)
        .display(Display::InlineFlex)
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .flex_shrink(0.0)
        .vertical_align(VerticalAlign::Middle)
        .width(rem(2))
        .height(rem(2))
        .overflow(Overflow::Hidden)
        .border_radius(px(999))
        .background_color(token::muted().var())
        .color(token::muted_foreground().var())
        .font_size(rem(0.875))
        .font_weight(FontWeight::Number(500.0))
}

/// The picture, filling the circle. Its `::after`, which a browser draws
/// only for an image that failed to load, covers the broken image with the
/// `alt` text.
fn picture_style() -> Style {
    Style::new()
        .position(Position::Relative)
        .display(Display::Block)
        .width(raw("100%"))
        .height(raw("100%"))
        .object_fit(ObjectFit::Cover)
        .pseudo(
            PseudoElement::After,
            Style::new()
                .content(raw("attr(alt)"))
                .position(Position::Absolute)
                .inset(px(0))
                .display(Display::Flex)
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .background_color(token::muted().var())
                .color(token::muted_foreground().var()),
        )
}

#[cfg(test)]
mod tests {
    use super::Avatar;

    struct Team;

    #[test]
    fn without_a_picture_it_shows_the_initials() {
        let html = Avatar::<Team>::new("Grace Hopper", "GH").render().render();
        assert!(html.contains(r#"role="img""#));
        assert!(html.contains(">GH</span>"));
        assert!(!html.contains("<img"));
    }
}
