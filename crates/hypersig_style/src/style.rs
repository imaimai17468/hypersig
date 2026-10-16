//! [`Style`], a set of declarations that becomes atomic classes.

use std::fmt::Write as _;

use crate::condition::{Condition, Conditions};
use crate::generated::{Property, PseudoElement};
use crate::value::Global;

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
}

/// One declaration and the conditions it applies under.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Atom {
    pub(crate) conditions: Conditions,
    pub(crate) property: Property,
    pub(crate) value: String,
}

impl Atom {
    /// The class name, from the conditions, the property, and the value, with
    /// `when:` in front for a toggled declaration.
    ///
    /// The value is written out where it uses only letters, digits, `.`, `#`,
    /// `%`, and `-`. Any other value is replaced by a 64-bit FNV-1a hash of it,
    /// so two different values never share a name unless their hashes collide.
    pub(crate) fn class_name(&self, toggled: bool) -> String {
        let mut name = String::new();
        if toggled {
            name.push_str("when:");
        }
        for m in &self.conditions.media {
            name.push_str(m.name());
            name.push(':');
        }
        for (_, p) in &self.conditions.pseudo {
            name.push_str(p.name());
            name.push(':');
        }
        if let Some(e) = self.conditions.element {
            name.push_str(e.name());
            name.push(':');
        }
        name.push_str(self.property.name());
        name.push('_');
        let readable = self
            .value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'#' | b'%' | b'-'));
        if readable {
            name.push_str(&self.value);
        } else {
            let _ = write!(name, "h{:016x}", fnv1a(self.value.as_bytes()));
        }
        name
    }
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
            property,
            value,
        });
        self
    }

    /// Sets `property` to a keyword every property accepts, such as `inherit`.
    #[must_use]
    pub fn global(self, property: Property, value: Global) -> Self {
        self.with(property, value.to_string())
    }

    /// Adds the declarations of `style`, which win over this style's own.
    #[must_use]
    pub fn merge(mut self, style: Self) -> Self {
        for atom in style.atoms {
            self.insert(atom);
        }
        self
    }

    /// Adds the declarations of `style`, applied only under `condition`.
    ///
    /// Conditions nest: `style` may use `on` itself, and its declarations then
    /// apply under both. The stylesheet writes the pseudo-classes of one
    /// declaration in a fixed order, so `on(Hover, on(Focus, ..))` and
    /// `on(Focus, on(Hover, ..))` produce the same class.
    #[must_use]
    pub fn on(mut self, condition: impl Into<Condition>, style: Self) -> Self {
        let condition = condition.into();
        for mut atom in style.atoms {
            atom.conditions.add(condition);
            self.insert(atom);
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
    /// because CSS has no pseudo-element of a pseudo-element.
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
        self
    }

    /// The class names an element needs for this style.
    pub fn class_names(&self) -> impl Iterator<Item = String> + '_ {
        self.atoms.iter().map(|a| a.class_name(false))
    }

    /// The class names for this style when it is switched on and off at run
    /// time. Their rules sit in a later cascade layer than those of
    /// [`Style::class_names`], so while switched on they override the
    /// element's own declarations for the same property.
    pub fn toggled_class_names(&self) -> impl Iterator<Item = String> + '_ {
        self.atoms.iter().map(|a| a.class_name(true))
    }

    /// Whether the style has no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }
}
