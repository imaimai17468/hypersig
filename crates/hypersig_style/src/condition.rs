//! The conditions a part of a [`Style`](crate::Style) applies under.

use crate::generated::{PseudoClass, PseudoElement};

/// A media feature a part of a style applies under.
///
/// The breakpoints are min-width queries in `rem`, so a style for one applies
/// from that width up, and a later breakpoint overrides an earlier one. Their
/// widths are the default breakpoints of Tailwind CSS and Panda CSS (640, 768,
/// 1024, 1280, and 1536 pixels at a 16px root font size), so a layout ported
/// from either keeps its widths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Media {
    /// From 40rem wide.
    Sm,
    /// From 48rem wide.
    Md,
    /// From 64rem wide.
    Lg,
    /// From 80rem wide.
    Xl,
    /// From 96rem wide.
    Xxl,
    /// The user prefers a dark color scheme.
    Dark,
    /// The user prefers a light color scheme.
    Light,
    /// The user asks for reduced motion.
    ReducedMotion,
    /// The page is printed.
    Print,
    /// The viewport is wider than it is tall.
    Landscape,
    /// The viewport is taller than it is wide.
    Portrait,
    /// The primary pointer can hover, as a mouse can and a finger cannot.
    CanHover,
    /// The primary pointer is coarse, such as a finger.
    CoarsePointer,
    /// The user agent forces a limited palette, as Windows high contrast does.
    ForcedColors,
    /// The user asks for more contrast.
    MoreContrast,
}

impl Media {
    /// The name this condition contributes to a class name.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "2xl",
            Self::Dark => "dark",
            Self::Light => "light",
            Self::ReducedMotion => "motion-reduce",
            Self::Print => "print",
            Self::Landscape => "landscape",
            Self::Portrait => "portrait",
            Self::CanHover => "can-hover",
            Self::CoarsePointer => "coarse-pointer",
            Self::ForcedColors => "forced-colors",
            Self::MoreContrast => "more-contrast",
        }
    }

    /// The media query, without `@media`.
    pub(crate) fn query(self) -> &'static str {
        match self {
            Self::Sm => "(min-width: 40rem)",
            Self::Md => "(min-width: 48rem)",
            Self::Lg => "(min-width: 64rem)",
            Self::Xl => "(min-width: 80rem)",
            Self::Xxl => "(min-width: 96rem)",
            Self::Dark => "(prefers-color-scheme: dark)",
            Self::Light => "(prefers-color-scheme: light)",
            Self::ReducedMotion => "(prefers-reduced-motion: reduce)",
            Self::Print => "print",
            Self::Landscape => "(orientation: landscape)",
            Self::Portrait => "(orientation: portrait)",
            Self::CanHover => "(hover: hover)",
            Self::CoarsePointer => "(pointer: coarse)",
            Self::ForcedColors => "(forced-colors: active)",
            Self::MoreContrast => "(prefers-contrast: more)",
        }
    }
}

/// A width of the nearest query container a part of a style applies from.
///
/// The widths are the default container sizes of Panda CSS, from 16rem to
/// 90rem. An ancestor becomes a query container with `container-type`, as
/// `Style::new().container_type(ContainerType::InlineSize)` sets it. Without
/// one, the condition never matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Container {
    /// From 16rem wide, Panda CSS's `@/3xs`.
    Xs3,
    /// From 18rem wide, Panda CSS's `@/2xs`.
    Xs2,
    /// From 20rem wide, Panda CSS's `@/xs`.
    Xs,
    /// From 24rem wide, Panda CSS's `@/sm`.
    Sm,
    /// From 28rem wide, Panda CSS's `@/md`.
    Md,
    /// From 32rem wide, Panda CSS's `@/lg`.
    Lg,
    /// From 36rem wide, Panda CSS's `@/xl`.
    Xl,
    /// From 42rem wide, Panda CSS's `@/2xl`.
    Xl2,
    /// From 48rem wide, Panda CSS's `@/3xl`.
    Xl3,
    /// From 56rem wide, Panda CSS's `@/4xl`.
    Xl4,
    /// From 64rem wide, Panda CSS's `@/5xl`.
    Xl5,
    /// From 72rem wide, Panda CSS's `@/6xl`.
    Xl6,
    /// From 80rem wide, Panda CSS's `@/7xl`.
    Xl7,
    /// From 90rem wide, Panda CSS's `@/8xl`.
    Xl8,
}

impl Container {
    /// The name this condition contributes to a class name.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Xs3 => "@3xs",
            Self::Xs2 => "@2xs",
            Self::Xs => "@xs",
            Self::Sm => "@sm",
            Self::Md => "@md",
            Self::Lg => "@lg",
            Self::Xl => "@xl",
            Self::Xl2 => "@2xl",
            Self::Xl3 => "@3xl",
            Self::Xl4 => "@4xl",
            Self::Xl5 => "@5xl",
            Self::Xl6 => "@6xl",
            Self::Xl7 => "@7xl",
            Self::Xl8 => "@8xl",
        }
    }

    /// The container query, without `@container`.
    pub(crate) fn query(self) -> &'static str {
        match self {
            Self::Xs3 => "(min-width: 16rem)",
            Self::Xs2 => "(min-width: 18rem)",
            Self::Xs => "(min-width: 20rem)",
            Self::Sm => "(min-width: 24rem)",
            Self::Md => "(min-width: 28rem)",
            Self::Lg => "(min-width: 32rem)",
            Self::Xl => "(min-width: 36rem)",
            Self::Xl2 => "(min-width: 42rem)",
            Self::Xl3 => "(min-width: 48rem)",
            Self::Xl4 => "(min-width: 56rem)",
            Self::Xl5 => "(min-width: 64rem)",
            Self::Xl6 => "(min-width: 72rem)",
            Self::Xl7 => "(min-width: 80rem)",
            Self::Xl8 => "(min-width: 90rem)",
        }
    }
}

/// The user-action pseudo-classes in the order they are written, so that a
/// later one wins while both match: `:active` over `:hover` while a button is
/// pressed, and `:disabled` over all of them. The order follows the
/// link-visited-hover-focus-active rule CSS authors use, extended with the
/// focus variants and `:disabled`.
const ACTION_ORDER: &[PseudoClass] = &[
    PseudoClass::Link,
    PseudoClass::Visited,
    PseudoClass::FocusWithin,
    PseudoClass::Hover,
    PseudoClass::Focus,
    PseudoClass::FocusVisible,
    PseudoClass::Active,
    PseudoClass::Disabled,
];

impl PseudoClass {
    /// Where a rule for this pseudo-class is written relative to others.
    pub(crate) fn rank(self) -> usize {
        ACTION_ORDER
            .iter()
            .position(|&p| p == self)
            .map_or(0, |i| i + 1)
    }
}

/// A condition that [`Style::on`](crate::Style::on) applies a part of a style under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    /// The element matches a pseudo-class.
    Pseudo(PseudoClass),
    /// The document matches a media feature.
    Media(Media),
    /// The nearest query container is at least this wide.
    Container(Container),
    /// An ancestor marked with [`Style::group`](crate::Style::group) matches
    /// a pseudo-class, as in `.group:hover .x`.
    Group(PseudoClass),
    /// An earlier sibling marked with [`Style::peer`](crate::Style::peer)
    /// matches a pseudo-class, as in `.peer:checked ~ .x`.
    Peer(PseudoClass),
}

impl From<Container> for Condition {
    fn from(c: Container) -> Self {
        Self::Container(c)
    }
}

impl From<PseudoClass> for Condition {
    fn from(p: PseudoClass) -> Self {
        Self::Pseudo(p)
    }
}

impl From<Media> for Condition {
    fn from(m: Media) -> Self {
        Self::Media(m)
    }
}

/// The conditions of one declaration, in the order the stylesheet writes them.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Conditions {
    /// Sorted by enum order, so breakpoints come out ascending.
    pub(crate) media: Vec<Media>,
    /// Sorted by enum order, so widths come out ascending.
    pub(crate) container: Vec<Container>,
    /// Sorted by [`PseudoClass::rank`], then enum order, as are `peer` and `pseudo`.
    pub(crate) group: Vec<(usize, PseudoClass)>,
    pub(crate) peer: Vec<(usize, PseudoClass)>,
    pub(crate) pseudo: Vec<(usize, PseudoClass)>,
    pub(crate) element: Option<PseudoElement>,
}

/// The class [`Style::group`](crate::Style::group) marks an ancestor with.
pub(crate) const GROUP: &str = "group";

/// The class [`Style::peer`](crate::Style::peer) marks a sibling with.
pub(crate) const PEER: &str = "peer";

/// Writes `rule`, which ends with a space, then `queries` joined by ` and `,
/// then the `{` that opens the block.
pub(crate) fn open_block(
    out: &mut String,
    rule: &str,
    queries: impl IntoIterator<Item = &'static str>,
) {
    out.push_str(rule);
    for (i, query) in queries.into_iter().enumerate() {
        if i > 0 {
            out.push_str(" and ");
        }
        out.push_str(query);
    }
    out.push('{');
}

fn insert_ranked(list: &mut Vec<(usize, PseudoClass)>, p: PseudoClass) {
    let key = (p.rank(), p);
    if let Err(at) = list.binary_search(&key) {
        list.insert(at, key);
    }
}

impl Conditions {
    pub(crate) fn add(&mut self, condition: Condition) {
        match condition {
            Condition::Media(m) => {
                if let Err(at) = self.media.binary_search(&m) {
                    self.media.insert(at, m);
                }
            }
            Condition::Container(c) => {
                if let Err(at) = self.container.binary_search(&c) {
                    self.container.insert(at, c);
                }
            }
            Condition::Pseudo(p) => insert_ranked(&mut self.pseudo, p),
            Condition::Group(p) => insert_ranked(&mut self.group, p),
            Condition::Peer(p) => insert_ranked(&mut self.peer, p),
        }
    }

    /// Whether a group or peer condition is present, which a selector
    /// without a class of its own cannot carry.
    pub(crate) fn has_relation(&self) -> bool {
        !self.group.is_empty() || !self.peer.is_empty()
    }

    /// Writes the selector for the element `target` writes: the group and
    /// peer prefixes, the target, then its pseudo-classes and pseudo-element.
    pub(crate) fn write_selector(&self, out: &mut String, target: impl FnOnce(&mut String)) {
        if !self.group.is_empty() {
            out.push('.');
            out.push_str(GROUP);
            for (_, p) in &self.group {
                out.push(':');
                out.push_str(p.name());
            }
            out.push(' ');
        }
        if !self.peer.is_empty() {
            out.push('.');
            out.push_str(PEER);
            for (_, p) in &self.peer {
                out.push(':');
                out.push_str(p.name());
            }
            out.push_str(" ~ ");
        }
        target(out);
        for (_, p) in &self.pseudo {
            out.push(':');
            out.push_str(p.name());
        }
        if let Some(e) = self.element {
            out.push_str("::");
            out.push_str(e.name());
        }
    }

    /// Opens the `@media` and `@container` blocks these conditions need, and
    /// returns how many [`Conditions::close`] has to close.
    pub(crate) fn open(&self, out: &mut String) -> usize {
        let mut open = 0;
        if !self.media.is_empty() {
            open_block(out, "@media ", self.media.iter().map(|m| m.query()));
            open += 1;
        }
        if !self.container.is_empty() {
            open_block(out, "@container ", self.container.iter().map(|c| c.query()));
            open += 1;
        }
        open
    }

    /// Closes the blocks [`Conditions::open`] opened.
    pub(crate) fn close(open: usize, out: &mut String) {
        for _ in 0..open {
            out.push('}');
        }
    }
}
