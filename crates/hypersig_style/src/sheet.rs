//! [`Sheet`], which collects styles and writes their rules.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use crate::condition::Conditions;
use crate::generated::Property;
use crate::style::{Atom, Style};

/// The cascade layer every atomic rule is written into.
///
/// A rule outside any layer beats a layered one, so a plain stylesheet a page
/// also loads overrides these classes.
pub const LAYER: &str = "hypersig-atoms";

/// The rules for a set of styles, each declaration written once.
///
/// ```
/// use hypersig_style::value::{Color, px};
/// use hypersig_style::{PseudoClass, Sheet, Style};
///
/// let mut sheet = Sheet::new();
/// sheet.add(&Style::new().padding(px(8)).color(Color::CRIMSON));
/// sheet.add(&Style::new().color(Color::CRIMSON).on(PseudoClass::Hover, Style::new().color(Color::RED)));
/// assert_eq!(
///     sheet.to_css(),
///     "@layer hypersig-atoms{.color_crimson{color:crimson}.padding_8px{padding:8px}\
///      .hover\\:color_red:hover{color:red}}"
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct Sheet {
    rules: BTreeSet<Key>,
}

/// The order rules are written in: by conditions, so that a rule under more or
/// later conditions comes after the rules it overrides, then by shorthand rank,
/// so that `padding` comes before `padding-top`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    conditions: Conditions,
    rank: u32,
    property: Property,
    value: String,
}

impl Sheet {
    /// An empty sheet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the rules `style` needs.
    pub fn add(&mut self, style: &Style) {
        for atom in style.atoms() {
            if !self.rules.iter().any(|k| k.matches(atom)) {
                self.rules.insert(Key {
                    conditions: atom.conditions.clone(),
                    rank: atom.property.rank(),
                    property: atom.property,
                    value: atom.value.clone(),
                });
            }
        }
    }

    /// Whether no style has been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Writes the rules into one `@layer` block.
    pub fn write_css(&self, out: &mut String) {
        if self.rules.is_empty() {
            return;
        }
        let _ = write!(out, "@layer {LAYER}{{");
        let mut open: Option<&[crate::Media]> = None;
        for key in &self.rules {
            let media = key.conditions.media.as_slice();
            if open != Some(media) {
                if open.is_some_and(|m| !m.is_empty()) {
                    out.push('}');
                }
                if !media.is_empty() {
                    out.push_str("@media ");
                    for (i, m) in media.iter().enumerate() {
                        if i > 0 {
                            out.push_str(" and ");
                        }
                        out.push_str(m.query());
                    }
                    out.push('{');
                }
                open = Some(media);
            }
            key.write_rule(out);
        }
        if open.is_some_and(|m| !m.is_empty()) {
            out.push('}');
        }
        out.push('}');
    }

    /// The rules as a string.
    #[must_use]
    pub fn to_css(&self) -> String {
        let mut out = String::new();
        self.write_css(&mut out);
        out
    }
}

impl Key {
    fn atom(&self) -> Atom {
        Atom {
            conditions: self.conditions.clone(),
            property: self.property,
            value: self.value.clone(),
        }
    }

    fn matches(&self, atom: &Atom) -> bool {
        self.property == atom.property
            && self.value == atom.value
            && self.conditions == atom.conditions
    }

    fn write_rule(&self, out: &mut String) {
        out.push('.');
        escape_class(&self.atom().class_name(), out);
        for (_, p) in &self.conditions.pseudo {
            out.push(':');
            out.push_str(p.name());
        }
        if let Some(e) = self.conditions.element {
            out.push_str("::");
            out.push_str(e.name());
        }
        let _ = write!(out, "{{{}:{}}}", self.property.name(), self.value);
    }
}

/// Writes `name` as the identifier after `.` in a selector, escaping what
/// `CSS.escape` escapes: a leading digit as a code point, and any other
/// character outside letters, digits, `-`, `_`, and non-ASCII with a backslash.
fn escape_class(name: &str, out: &mut String) {
    for (i, c) in name.chars().enumerate() {
        if i == 0 && c.is_ascii_digit() {
            let _ = write!(out, "\\{:x} ", u32::from(c));
        } else if c.is_ascii_alphanumeric() || c == '-' || c == '_' || !c.is_ascii() {
            out.push(c);
        } else {
            out.push('\\');
            out.push(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sheet, escape_class};
    use crate::value::{Color, Display, pct, px, quoted, raw, rem};
    use crate::{Media, PseudoClass, PseudoElement, Style};

    #[test]
    fn escapes_a_leading_digit_and_punctuation() {
        let mut out = String::new();
        escape_class("2xl:width_50%", &mut out);
        assert_eq!(out, "\\32 xl\\:width_50\\%");
    }

    #[test]
    fn writes_breakpoints_ascending_after_the_base_rules() {
        let mut sheet = Sheet::new();
        sheet.add(
            &Style::new()
                .on(Media::Lg, Style::new().width(pct(25)))
                .on(Media::Md, Style::new().width(pct(50)))
                .width(pct(100)),
        );
        assert_eq!(
            sheet.to_css(),
            "@layer hypersig-atoms{.width_100\\%{width:100%}\
             @media (min-width: 48rem){.md\\:width_50\\%{width:50%}}\
             @media (min-width: 64rem){.lg\\:width_25\\%{width:25%}}}"
        );
    }

    #[test]
    fn writes_shorthands_before_longhands() {
        let mut sheet = Sheet::new();
        sheet.add(&Style::new().padding_top(px(0)).padding(rem(1)));
        assert_eq!(
            sheet.to_css(),
            "@layer hypersig-atoms{.padding_1rem{padding:1rem}.padding-top_0{padding-top:0}}"
        );
    }

    #[test]
    fn writes_active_after_hover() {
        let mut sheet = Sheet::new();
        sheet.add(
            &Style::new()
                .on(PseudoClass::Active, Style::new().color(Color::RED))
                .on(PseudoClass::Hover, Style::new().color(Color::BLUE)),
        );
        assert_eq!(
            sheet.to_css(),
            "@layer hypersig-atoms{.hover\\:color_blue:hover{color:blue}\
             .active\\:color_red:active{color:red}}"
        );
    }

    #[test]
    fn hashes_values_with_spaces_and_writes_pseudo_elements_last() {
        let style = Style::new()
            .display(Display::Flex)
            .margin(raw("0 auto"))
            .on(
                PseudoClass::Hover,
                Style::new().pseudo(PseudoElement::After, Style::new().content(quoted("×"))),
            );
        let names: Vec<String> = style.class_names().collect();
        assert_eq!(names[0], "display_flex");
        assert!(names[1].starts_with("margin_h") && names[1].len() == "margin_h".len() + 16);
        let mut sheet = Sheet::new();
        sheet.add(&style);
        assert!(sheet.to_css().contains(":hover::after{content:\"×\"}"));
    }

    #[test]
    fn a_later_value_replaces_an_earlier_one() {
        let style = Style::new()
            .color(Color::RED)
            .merge(Style::new().color(Color::BLUE));
        assert_eq!(style.class_names().collect::<Vec<_>>(), ["color_blue"]);
    }
}
