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
    /// Sorted by [`PseudoClass::rank`], then enum order.
    pub(crate) pseudo: Vec<(usize, PseudoClass)>,
    pub(crate) element: Option<PseudoElement>,
}

impl Conditions {
    pub(crate) fn add(&mut self, condition: Condition) {
        match condition {
            Condition::Media(m) => {
                if let Err(at) = self.media.binary_search(&m) {
                    self.media.insert(at, m);
                }
            }
            Condition::Pseudo(p) => {
                let key = (p.rank(), p);
                if let Err(at) = self.pseudo.binary_search(&key) {
                    self.pseudo.insert(at, key);
                }
            }
        }
    }
}
