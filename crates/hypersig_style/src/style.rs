//! [`Style`], a set of declarations that becomes atomic classes.

use std::fmt::Write as _;

use crate::condition::{Condition, Conditions, GROUP, PEER};
use crate::generated::{Property, PseudoElement};
use crate::keyframes::Keyframes;
use crate::token::{Token, TokenValue};
use crate::value::CssWideKeyword;

/// A set of CSS declarations, each of which becomes one class.
///
/// A property method replaces an earlier value for the same property under the
/// same conditions, so a later call wins as it would later in a rule.
///
/// ```
/// use hypersig_style::value::{Color, Display, pct, px, quoted, rem};
/// use hypersig_style::{Media, PseudoClass, PseudoElement, Style};
///
/// let card = Style::new()
///     .display(Display::Flex)
///     .padding(rem(1))
///     .color(Color::hex(0x222222))
///     .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON))
///     .on(Media::Md, Style::new().width(pct(50)))
///     .pseudo(PseudoElement::After, Style::new().content(quoted("→")).margin_left(px(4)));
/// let names: Vec<String> = card.class_names().collect();
/// assert!(names.contains(&"hover:color_crimson".to_owned()));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    atoms: Vec<Atom>,
    keyframes: Vec<Keyframes>,
    markers: Vec<&'static str>,
}

/// One declaration and the conditions it applies under.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Atom {
    pub(crate) conditions: Conditions,
    pub(crate) property: Prop,
    pub(crate) value: String,
}

/// The property of a declaration: a standard one, or the custom property of
/// a [`Token`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Prop {
    Standard(Property),
    Custom(&'static str),
}

impl Prop {
    /// The name after which the declaration's class is named, which for a
    /// custom property starts with `--`.
    pub(crate) fn write_name(self, out: &mut String) {
        match self {
            Self::Standard(p) => out.push_str(p.name()),
            Self::Custom(name) => {
                out.push_str("--");
                out.push_str(name);
            }
        }
    }

    /// How many shorthands stand above the property. A custom property has none.
    pub(crate) fn rank(self) -> u32 {
        match self {
            Self::Standard(p) => p.rank(),
            Self::Custom(_) => 0,
        }
    }
}

impl Atom {
    /// The class name of this declaration, as [`class_name`] writes it.
    pub(crate) fn class_name(&self, toggled: bool) -> String {
        class_name(&self.conditions, self.property, &self.value, toggled)
    }
}

/// The class name, from the conditions, the property, and the value, with
/// `when:` in front for a toggled declaration.
///
/// The value is written out where it uses only letters, digits, `.`, `#`,
/// `%`, and `-`. Any other value is replaced by a 64-bit FNV-1a hash of it,
/// so two different values never share a name unless their hashes collide.
pub(crate) fn class_name(
    conditions: &Conditions,
    property: Prop,
    value: &str,
    toggled: bool,
) -> String {
    let mut name = String::new();
    if toggled {
        name.push_str("when:");
    }
    for m in &conditions.media {
        name.push_str(m.name());
        name.push(':');
    }
    for c in &conditions.container {
        name.push_str(c.name());
        name.push(':');
    }
    for (_, p) in &conditions.group {
        name.push_str("group-");
        name.push_str(p.name());
        name.push(':');
    }
    for (_, p) in &conditions.peer {
        name.push_str("peer-");
        name.push_str(p.name());
        name.push(':');
    }
    for (_, p) in &conditions.pseudo {
        name.push_str(p.name());
        name.push(':');
    }
    if let Some(e) = conditions.element {
        name.push_str(e.name());
        name.push(':');
    }
    property.write_name(&mut name);
    name.push('_');
    let readable = value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'#' | b'%' | b'-'));
    if readable {
        name.push_str(value);
    } else {
        let _ = write!(name, "h{:016x}", fnv1a(value.as_bytes()));
    }
    name
}

/// The 64-bit FNV-1a hash, chosen because it is a few lines and stable across
/// Rust versions, which `std`'s `DefaultHasher` does not promise.
fn fnv1a(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0100_0000_01b3;
    bytes
        .iter()
        .fold(OFFSET, |h, &b| (h ^ u64::from(b)).wrapping_mul(PRIME))
}

impl Style {
    /// An empty style.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    pub(crate) fn keyframes_used(&self) -> &[Keyframes] {
        &self.keyframes
    }

    /// Animates the element with `keyframes`.
    ///
    /// It sets `animation-name` to its name, and carries the `@keyframes` rule
    /// to the page.
    ///
    /// # Panics
    ///
    /// Panics if the style already carries other keyframes with the same
    /// name and other steps.
    #[must_use]
    pub fn keyframes(mut self, keyframes: &Keyframes) -> Self {
        self.carry(keyframes.clone());
        self.with(Property::AnimationName, keyframes.name().to_owned())
    }

    /// Adds `keyframes` to the rules the style carries, once per name.
    fn carry(&mut self, keyframes: Keyframes) {
        match self.keyframes.iter().find(|k| k.name() == keyframes.name()) {
            Some(old) => {
                Keyframes::assert_one_rule(old.name(), &old.to_css(), &keyframes.to_css());
            }
            None => self.keyframes.push(keyframes),
        }
    }

    fn insert(&mut self, atom: Atom) {
        if let Some(old) = self
            .atoms
            .iter_mut()
            .find(|a| a.property == atom.property && a.conditions == atom.conditions)
        {
            *old = atom;
        } else {
            self.atoms.push(atom);
        }
    }

    pub(crate) fn with(mut self, property: Property, value: String) -> Self {
        self.insert(Atom {
            conditions: Conditions::default(),
            property: Prop::Standard(property),
            value,
        });
        self
    }

    /// Sets `token` to `value` on the element and everything inside it.
    ///
    /// It takes the place of the default and the page's
    /// [`Theme`](crate::Theme).
    #[must_use]
    pub fn token<T: TokenValue>(mut self, token: &Token<T>, value: impl Into<T>) -> Self {
        self.insert(Atom {
            conditions: Conditions::default(),
            property: Prop::Custom(token.name()),
            value: value.into().to_string(),
        });
        self
    }

    /// Sets `property` to a keyword every property accepts, such as `inherit`.
    #[must_use]
    pub fn keyword(self, property: Property, value: CssWideKeyword) -> Self {
        self.with(property, value.to_string())
    }

    /// Adds the declarations of `style`, which win over this style's own.
    ///
    /// # Panics
    ///
    /// Panics if both styles carry keyframes with one name and other steps,
    /// as [`Style::keyframes`] does.
    #[must_use]
    pub fn merge(mut self, style: Self) -> Self {
        for atom in style.atoms {
            self.insert(atom);
        }
        for keyframes in style.keyframes {
            self.carry(keyframes);
        }
        for marker in style.markers {
            self.mark(marker);
        }
        self
    }

    fn mark(&mut self, marker: &'static str) {
        if !self.markers.contains(&marker) {
            self.markers.push(marker);
        }
    }

    /// Adds the declarations of `style`, applied only under `condition`.
    ///
    /// Conditions nest: `style` may use `on` itself, and its declarations then
    /// apply under both. The stylesheet writes the pseudo-classes of one
    /// declaration in a fixed order, so `on(Hover, on(Focus, ..))` and
    /// `on(Focus, on(Hover, ..))` produce the same class.
    ///
    /// # Panics
    ///
    /// Panics as [`Style::merge`] does.
    #[must_use]
    pub fn on(mut self, condition: impl Into<Condition>, style: Self) -> Self {
        let condition = condition.into();
        for mut atom in style.atoms {
            atom.conditions.add(condition);
            self.insert(atom);
        }
        for keyframes in style.keyframes {
            self.carry(keyframes);
        }
        self
    }

    /// Adds the declarations of `style` to the pseudo-element `element`.
    ///
    /// Pseudo-classes in `style` apply to the element, so
    /// `pseudo(Before, Style::new().on(Hover, ..))` is written
    /// `:hover::before`.
    ///
    /// # Panics
    ///
    /// Panics if `style` already places a declaration on a pseudo-element,
    /// because CSS has no pseudo-element of a pseudo-element, and as
    /// [`Style::merge`] does.
    #[must_use]
    pub fn pseudo(mut self, element: PseudoElement, style: Self) -> Self {
        for mut atom in style.atoms {
            assert!(
                atom.conditions.element.is_none(),
                "a pseudo-element holds no pseudo-element"
            );
            atom.conditions.element = Some(element);
            self.insert(atom);
        }
        for keyframes in style.keyframes {
            self.carry(keyframes);
        }
        self
    }

    /// Marks the element as the ancestor that [`Condition::Group`] tests,
    /// with the class `group`. Any marked ancestor that matches counts, as
    /// with the `group` of Tailwind CSS and Panda CSS.
    #[must_use]
    pub fn group(mut self) -> Self {
        self.mark(GROUP);
        self
    }

    /// Marks the element as the earlier sibling that [`Condition::Peer`]
    /// tests, with the class `peer`.
    #[must_use]
    pub fn peer(mut self) -> Self {
        self.mark(PEER);
        self
    }

    /// The class names an element needs for this style.
    pub fn class_names(&self) -> impl Iterator<Item = String> + '_ {
        self.markers
            .iter()
            .map(|&m| m.to_owned())
            .chain(self.atoms.iter().map(|a| a.class_name(false)))
    }

    /// The class names for this style when it is toggled at run time.
    ///
    /// Their rules sit in a later cascade layer than those of
    /// [`Style::class_names`], so while switched on they override the element's
    /// own declarations for the same property.
    pub fn toggled_class_names(&self) -> impl Iterator<Item = String> + '_ {
        self.markers
            .iter()
            .map(|&m| m.to_owned())
            .chain(self.atoms.iter().map(|a| a.class_name(true)))
    }

    /// Whether the style has no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty() && self.markers.is_empty()
    }
}
