//! [`Global`]: rules for elements by their tag, a reset, and `@font-face`.

use std::fmt::Write as _;

use crate::Property;
use crate::condition::Conditions;
use crate::sheet::{BASE_LAYER, ORDER};
use crate::style::Style;
use crate::value::{
    BoxSizing, Color, CssWideKeyword, Display, FontFamily, LineHeight, Quoted, Url, pct, px, quoted,
};

/// What a [`Global`] rule applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Selector {
    /// The document's root, `:root`.
    Root,
    /// Every element and its `::before` and `::after`.
    Everything,
    /// Every element with the tag.
    Element(Element),
}

impl Selector {
    fn write(self, out: &mut String, suffix: &str) {
        match self {
            Self::Root => {
                let _ = write!(out, ":root{suffix}");
            }
            Self::Everything => {
                let _ = write!(out, "*{suffix},::before{suffix},::after{suffix}");
            }
            Self::Element(e) => {
                let _ = write!(out, "{}{suffix}", e.name());
            }
        }
    }
}

impl From<Element> for Selector {
    fn from(e: Element) -> Self {
        Self::Element(e)
    }
}

macro_rules! elements {
    ($($variant:ident => $tag:literal),* $(,)?) => {
        /// An HTML element, by its tag.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum Element {
            $(
                #[doc = concat!("`<", $tag, ">`.")]
                $variant,
            )*
        }

        impl Element {
            /// The tag name.
            #[must_use]
            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $tag,)*
                }
            }
        }
    };
}

elements! {
    A => "a", Abbr => "abbr", Address => "address", Article => "article", Aside => "aside",
    Audio => "audio", B => "b", Blockquote => "blockquote", Body => "body", Button => "button",
    Canvas => "canvas", Caption => "caption", Code => "code", Dd => "dd", Del => "del",
    Details => "details", Dialog => "dialog", Div => "div", Dl => "dl", Dt => "dt", Em => "em",
    Fieldset => "fieldset", Figcaption => "figcaption", Figure => "figure", Footer => "footer",
    Form => "form", H1 => "h1", H2 => "h2", H3 => "h3", H4 => "h4", H5 => "h5", H6 => "h6",
    Header => "header", Hr => "hr", Html => "html", I => "i", Iframe => "iframe", Img => "img",
    Input => "input", Ins => "ins", Kbd => "kbd", Label => "label", Legend => "legend",
    Li => "li", Main => "main", Mark => "mark", Menu => "menu", Meter => "meter", Nav => "nav",
    Ol => "ol", Optgroup => "optgroup", Option => "option", Output => "output", P => "p",
    Picture => "picture", Pre => "pre", Progress => "progress", Q => "q", S => "s",
    Samp => "samp", Search => "search", Section => "section", Select => "select",
    Small => "small", Span => "span", Strong => "strong", Sub => "sub", Summary => "summary",
    Sup => "sup", Svg => "svg", Table => "table", Tbody => "tbody", Td => "td",
    Textarea => "textarea", Tfoot => "tfoot", Th => "th", Thead => "thead", Time => "time",
    Tr => "tr", U => "u", Ul => "ul", Video => "video",
}

/// A font the page loads, written as `@font-face`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFace {
    family: &'static str,
    src: Url,
    weight: Option<(u16, u16)>,
    italic: bool,
}

impl FontFace {
    /// The font `family` loaded from `src`.
    ///
    /// `src` is a WOFF2 file or any format the browser recognizes from the
    /// file.
    #[must_use]
    pub fn new(family: &'static str, src: Url) -> Self {
        Self {
            family,
            src,
            weight: None,
            italic: false,
        }
    }

    /// The weights the file covers, from `min` to `max`.
    ///
    /// This is how a variable font declares them. One weight is `weights(400,
    /// 400)`.
    ///
    /// # Panics
    ///
    /// Panics if `min` is over `max`, or either is outside the 1 to 1000
    /// that CSS allows, where the browser would drop the descriptor.
    #[must_use]
    pub fn weights(mut self, min: u16, max: u16) -> Self {
        assert!(
            (1..=1000).contains(&min) && (1..=1000).contains(&max) && min <= max,
            "font weights run from 1 to 1000 with min <= max, not {min} to {max}"
        );
        self.weight = Some((min, max));
        self
    }

    /// Marks the file as the italic face of the family.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// The family as `font-family` takes it.
    #[must_use]
    pub fn family(&self) -> FontFamily {
        FontFamily::Quoted(quoted(self.family))
    }

    fn write(&self, out: &mut String) {
        let family: Quoted = quoted(self.family);
        let _ = write!(
            out,
            "@font-face{{font-family:{family};src:{};font-display:swap",
            self.src
        );
        if let Some((min, max)) = self.weight {
            let _ = write!(out, ";font-weight:{min} {max}");
        }
        if self.italic {
            out.push_str(";font-style:italic");
        }
        out.push('}');
    }
}

/// Rules for elements by their tag, and the fonts the page loads.
///
/// The rules sit in [`BASE_LAYER`], before the classes of
/// [`Style`]s, so an element's own style overrides them. Conditions in a
/// rule's style apply to the selector as they do to a class.
///
/// ```
/// use hypersig_style::value::{Color, FontFamily, rem};
/// use hypersig_style::{Element, Global, PseudoClass, Style};
///
/// let global = Global::preflight()
///     .rule(Element::Body, Style::new().font_family(FontFamily::SystemUi))
///     .rule(
///         Element::A,
///         Style::new().color(Color::CRIMSON).on(PseudoClass::Hover, Style::new().color(Color::RED)),
///     );
/// let css = global.to_css();
/// assert!(css.contains("body{font-family:system-ui;line-height:1.5}"));
/// assert!(css.contains("a:hover{color:red}"));
/// # let _ = rem(1);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Global {
    rules: Vec<(Selector, Style)>,
    fonts: Vec<FontFace>,
}

impl Global {
    /// No rules and no fonts.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A small reset.
    ///
    /// It sets `border-box` sizing everywhere, removes default margins, puts
    /// form controls in the page's font, and makes media shrink to its box.
    #[must_use]
    pub fn preflight() -> Self {
        Self::new()
            .rule(
                Selector::Everything,
                Style::new().box_sizing(BoxSizing::BorderBox).margin(px(0)),
            )
            .rule(
                Element::Body,
                Style::new().line_height(LineHeight::Number(1.5)),
            )
            .rule(
                Element::Img,
                Style::new().display(Display::Block).max_width(pct(100)),
            )
            .rule(
                Element::Svg,
                Style::new().display(Display::Block).max_width(pct(100)),
            )
            .rule(
                Element::Video,
                Style::new().display(Display::Block).max_width(pct(100)),
            )
            .rule(Element::Button, Self::inherit_font())
            .rule(Element::Input, Self::inherit_font())
            .rule(Element::Select, Self::inherit_font())
            .rule(Element::Textarea, Self::inherit_font())
    }

    fn inherit_font() -> Style {
        Style::new()
            .keyword(Property::Font, CssWideKeyword::Inherit)
            .color(Color::CURRENT_COLOR)
    }

    /// Adds `style` for every element `selector` matches. A later rule for the
    /// same selector adds to the earlier one, and wins where both set a
    /// property under the same conditions.
    #[must_use]
    pub fn rule(mut self, selector: impl Into<Selector>, style: Style) -> Self {
        let selector = selector.into();
        if let Some((_, old)) = self.rules.iter_mut().find(|(s, _)| *s == selector) {
            *old = std::mem::take(old).merge(style);
        } else {
            self.rules.push((selector, style));
        }
        self
    }

    /// Loads `font`.
    #[must_use]
    pub fn font(mut self, font: FontFace) -> Self {
        self.fonts.push(font);
        self
    }

    /// Adds the rules and fonts of `global`, whose rules win over this one's.
    #[must_use]
    pub fn merge(mut self, global: Self) -> Self {
        for (selector, style) in global.rules {
            self = self.rule(selector, style);
        }
        self.fonts.extend(global.fonts);
        self
    }

    /// Whether there are no rules and no fonts.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.fonts.is_empty()
    }

    /// The rules in [`BASE_LAYER`], then the `@font-face` rules.
    ///
    /// # Panics
    ///
    /// Panics if a rule's style uses a pseudo-element with
    /// [`Selector::Everything`], which already names `::before` and
    /// `::after`, or a [`Condition::Group`](crate::Condition::Group) or
    /// [`Condition::Peer`](crate::Condition::Peer), which need a marked
    /// element the selector does not name.
    #[must_use]
    pub fn to_css(&self) -> String {
        let mut out = format!("{ORDER}@layer {BASE_LAYER}{{");
        for (selector, style) in &self.rules {
            let mut sorted: Vec<_> = style.atoms().iter().collect();
            sorted.sort_by(|a, b| {
                (&a.conditions, a.property.rank(), a.property).cmp(&(
                    &b.conditions,
                    b.property.rank(),
                    b.property,
                ))
            });
            let mut i = 0;
            while i < sorted.len() {
                let conditions = &sorted[i].conditions;
                assert!(
                    !conditions.has_relation(),
                    "a global rule takes no group or peer condition"
                );
                let mut suffix = String::new();
                for (_, p) in &conditions.pseudo {
                    suffix.push(':');
                    suffix.push_str(p.name());
                }
                if let Some(e) = conditions.element {
                    assert!(
                        *selector != Selector::Everything,
                        "Selector::Everything already covers ::before and ::after"
                    );
                    suffix.push_str("::");
                    suffix.push_str(e.name());
                }
                let open = conditions.open(&mut out);
                selector.write(&mut out, &suffix);
                out.push('{');
                let mut first = true;
                while i < sorted.len() && sorted[i].conditions == *conditions {
                    if !first {
                        out.push(';');
                    }
                    first = false;
                    sorted[i].property.write_name(&mut out);
                    let _ = write!(out, ":{}", sorted[i].value);
                    i += 1;
                }
                out.push('}');
                Conditions::close(open, &mut out);
            }
        }
        out.push('}');
        for font in &self.fonts {
            font.write(&mut out);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{Element, FontFace, Global, Selector};
    use crate::value::{Color, url};
    use crate::{Media, Style};

    #[test]
    fn writes_the_preflight_in_the_base_layer() {
        let css = Global::preflight().to_css();
        assert!(css.starts_with(
            "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;\
             @layer hypersig-base{*,::before,::after{box-sizing:border-box;margin:0}"
        ));
        assert!(css.contains("button{color:currentColor;font:inherit}"));
    }

    #[test]
    fn writes_media_conditions_and_fonts() {
        let inter = FontFace::new("Inter", url("/inter.woff2")).weights(100, 900);
        let css = Global::new()
            .rule(
                Selector::Root,
                Style::new().on(Media::Dark, Style::new().color(Color::WHITE)),
            )
            .rule(Element::Body, Style::new().font_family(inter.family()))
            .font(inter)
            .to_css();
        assert!(css.contains(
            "@media (prefers-color-scheme: dark){:root{color:white}}body{font-family:\"Inter\"}}"
        ));
        assert!(css.ends_with(
            "@font-face{font-family:\"Inter\";src:url(\"/inter.woff2\");font-display:swap;font-weight:100 900}"
        ));
    }

    #[test]
    #[should_panic(expected = "font weights run from 1 to 1000 with min <= max, not 900 to 100")]
    fn weights_in_reverse_panic() {
        let _ = FontFace::new("Inter", url("/inter.woff2")).weights(900, 100);
    }

    #[test]
    #[should_panic(expected = "not 100 to 1200")]
    fn a_weight_above_1000_panics() {
        let _ = FontFace::new("Inter", url("/inter.woff2")).weights(100, 1200);
    }

    #[test]
    #[should_panic(expected = "not 0 to 400")]
    fn a_weight_of_0_panics() {
        let _ = FontFace::new("Inter", url("/inter.woff2")).weights(0, 400);
    }
}
