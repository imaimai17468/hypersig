//! [`Sheet`], which collects styles and writes their rules.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use crate::condition::Conditions;
use crate::style::{Atom, Prop, Style};

/// The cascade layer of the classes from [`Sheet::add`].
pub const LAYER: &str = "hypersig-atoms";

/// The cascade layer of the classes from [`Sheet::add_toggled`], which comes
/// after [`LAYER`], so a toggled declaration overrides an element's own.
pub const TOGGLED_LAYER: &str = "hypersig-toggled";

/// The cascade layer of a [`Theme`](crate::Theme)'s values, before [`LAYER`].
pub const TOKEN_LAYER: &str = "hypersig-tokens";

/// The statement that puts [`TOKEN_LAYER`], [`LAYER`], and [`TOGGLED_LAYER`]
/// in that order. Every bucket and theme starts with it, because the first
/// stylesheet to name a layer fixes its place, and any of them may be the
/// first one a page has.
pub(crate) const ORDER: &str = "@layer hypersig-tokens,hypersig-atoms,hypersig-toggled;";

/// The rules for a set of styles, each declaration written once.
///
/// A rule outside any cascade layer beats a layered one, so a plain stylesheet
/// the page also loads overrides these classes.
///
/// ```
/// use hypersig_style::value::{Color, px};
/// use hypersig_style::{PseudoClass, Sheet, Style};
///
/// let mut sheet = Sheet::new();
/// sheet.add(&Style::new().padding(px(8)).color(Color::CRIMSON));
/// sheet.add(&Style::new().on(PseudoClass::Hover, Style::new().color(Color::RED)));
/// let css: Vec<String> = sheet.buckets().map(|b| b.css().to_owned()).collect();
/// assert_eq!(
///     css,
///     [
///         "@layer hypersig-tokens,hypersig-atoms,hypersig-toggled;\
///          @layer hypersig-atoms.k--00{.color_crimson{color:crimson}.padding_8px{padding:8px}}",
///         "@layer hypersig-tokens,hypersig-atoms,hypersig-toggled;\
///          @layer hypersig-atoms.k-4019-00{.hover\\:color_red:hover{color:red}}",
///     ]
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct Sheet {
    rules: BTreeSet<Key>,
}

/// The order rules are written in: toggled after untoggled, then by
/// conditions, so that a rule under more or later conditions comes after the
/// rules it overrides, then by shorthand rank, so that `padding` comes before
/// `padding-top`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    toggled: bool,
    conditions: Conditions,
    rank: u32,
    property: Prop,
    value: String,
}

/// The rules of one place in the order: one cascade layer, one set of
/// conditions, and one shorthand rank.
///
/// Rules in one bucket never set the same property on the same element, so
/// their order among themselves does not matter. A page writes each bucket as
/// its own `<style>` element, so the runtime can add the rules a patch brings
/// to the right bucket, or insert a new bucket at its place in the order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    toggled: bool,
    key: String,
    css: String,
}

impl Bucket {
    /// The cascade layer, [`LAYER`] or [`TOGGLED_LAYER`].
    #[must_use]
    pub fn layer(&self) -> &'static str {
        if self.toggled { TOGGLED_LAYER } else { LAYER }
    }

    /// The sublayer name, which sorts as a string the way the buckets of one
    /// layer are ordered.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// A string that sorts the way buckets are ordered across both layers:
    /// `0` or `1` for the layer, a space, and [`Bucket::key`].
    #[must_use]
    pub fn sort_key(&self) -> String {
        format!("{} {}", u8::from(self.toggled), self.key)
    }

    /// The bucket's rules as a stylesheet of their own: the order of the two
    /// layers, then the rules inside `@layer <layer>.<key>{...}`.
    #[must_use]
    pub fn css(&self) -> &str {
        &self.css
    }
}

impl Sheet {
    /// An empty sheet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, style: &Style, toggled: bool) {
        for atom in style.atoms() {
            self.rules.insert(Key {
                toggled,
                conditions: atom.conditions.clone(),
                rank: atom.property.rank(),
                property: atom.property,
                value: atom.value.clone(),
            });
        }
    }

    /// Adds the rules for the classes [`Style::class_names`] lists.
    pub fn add(&mut self, style: &Style) {
        self.insert(style, false);
    }

    /// Adds the rules for the classes [`Style::toggled_class_names`] lists.
    pub fn add_toggled(&mut self, style: &Style) {
        self.insert(style, true);
    }

    /// Adds every rule of `other`.
    pub fn extend(&mut self, other: &Self) {
        self.rules.extend(other.rules.iter().cloned());
    }

    /// Whether no style has been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// The rules grouped into buckets, in the order they apply.
    pub fn buckets(&self) -> impl Iterator<Item = Bucket> + '_ {
        let mut rules = self.rules.iter().peekable();
        std::iter::from_fn(move || {
            let first = rules.next()?;
            let layer = if first.toggled { TOGGLED_LAYER } else { LAYER };
            let key = first.bucket();
            let mut css = format!("{ORDER}@layer {layer}.{key}{{");
            let media = first.conditions.media.as_slice();
            if !media.is_empty() {
                css.push_str("@media ");
                for (i, m) in media.iter().enumerate() {
                    if i > 0 {
                        css.push_str(" and ");
                    }
                    css.push_str(m.query());
                }
                css.push('{');
            }
            first.write_rule(&mut css);
            while let Some(next) = rules.next_if(|k| k.same_bucket(first)) {
                next.write_rule(&mut css);
            }
            if !media.is_empty() {
                css.push('}');
            }
            css.push('}');
            Some(Bucket {
                toggled: first.toggled,
                key,
                css,
            })
        })
    }

    /// Writes every bucket, as one stylesheet.
    pub fn write_css(&self, out: &mut String) {
        for bucket in self.buckets() {
            out.push_str(&bucket.css);
        }
    }

    /// The stylesheet [`Sheet::write_css`] writes.
    #[must_use]
    pub fn to_css(&self) -> String {
        let mut out = String::new();
        self.write_css(&mut out);
        out
    }
}

impl Key {
    fn same_bucket(&self, other: &Self) -> bool {
        self.toggled == other.toggled
            && self.conditions == other.conditions
            && self.rank == other.rank
    }

    /// The sublayer name: `k`, then one fixed-width field per condition and
    /// the shorthand rank, so that comparing two names as strings compares
    /// the buckets. Each list of conditions ends with `-`, which sorts before
    /// the digits, so a list sorts before any longer list it starts.
    fn bucket(&self) -> String {
        let mut key = String::from("k");
        for &m in &self.conditions.media {
            let _ = write!(key, "{:02}", m as u8);
        }
        key.push('-');
        for &(rank, p) in &self.conditions.pseudo {
            let _ = write!(key, "{rank}{:03}", p as u16);
        }
        key.push('-');
        match self.conditions.element {
            Some(e) => {
                let _ = write!(key, "1{:03}", e as u16);
            }
            None => key.push('0'),
        }
        let _ = write!(key, "{}", self.rank);
        key
    }

    fn write_rule(&self, out: &mut String) {
        let atom = Atom {
            conditions: self.conditions.clone(),
            property: self.property,
            value: self.value.clone(),
        };
        out.push('.');
        escape_class(&atom.class_name(self.toggled), out);
        for (_, p) in &self.conditions.pseudo {
            out.push(':');
            out.push_str(p.name());
        }
        if let Some(e) = self.conditions.element {
            out.push_str("::");
            out.push_str(e.name());
        }
        out.push('{');
        self.property.write_name(out);
        let _ = write!(out, ":{}}}", self.value);
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

    fn css(sheet: &Sheet) -> Vec<String> {
        sheet.buckets().map(|b| b.css().to_owned()).collect()
    }

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
        let css = css(&sheet);
        assert_eq!(css.len(), 3);
        assert!(css[0].ends_with("{.width_100\\%{width:100%}}"));
        assert!(css[1].ends_with("{@media (min-width: 48rem){.md\\:width_50\\%{width:50%}}}"));
        assert!(css[2].ends_with("{@media (min-width: 64rem){.lg\\:width_25\\%{width:25%}}}"));
    }

    #[test]
    fn bucket_keys_sort_as_the_buckets_do() {
        let mut sheet = Sheet::new();
        sheet.add(
            &Style::new()
                .on(
                    Media::Md,
                    Style::new().on(PseudoClass::Hover, Style::new().color(Color::RED)),
                )
                .on(Media::Md, Style::new().color(Color::BLUE))
                .on(PseudoClass::Active, Style::new().color(Color::GREEN))
                .padding_top(px(0))
                .padding(px(1)),
        );
        sheet.add_toggled(&Style::new().color(Color::GRAY));
        let keys: Vec<String> = sheet.buckets().map(|b| b.sort_key()).collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
        assert_eq!(keys.len(), 6);
    }

    #[test]
    fn writes_shorthands_before_longhands() {
        let mut sheet = Sheet::new();
        sheet.add(&Style::new().padding_top(px(0)).padding(rem(1)));
        let css = css(&sheet);
        assert!(css[0].ends_with("{.padding_1rem{padding:1rem}}"));
        assert!(css[1].ends_with("{.padding-top_0{padding-top:0}}"));
    }

    #[test]
    fn writes_active_after_hover() {
        let mut sheet = Sheet::new();
        sheet.add(
            &Style::new()
                .on(PseudoClass::Active, Style::new().color(Color::RED))
                .on(PseudoClass::Hover, Style::new().color(Color::BLUE)),
        );
        let css = css(&sheet);
        assert!(css[0].contains(":hover{color:blue}") && css[1].contains(":active{color:red}"));
    }

    #[test]
    fn toggled_classes_have_their_own_names_and_layer() {
        let style = Style::new().color(Color::RED);
        assert_eq!(
            style.toggled_class_names().collect::<Vec<_>>(),
            ["when:color_red"]
        );
        let mut sheet = Sheet::new();
        sheet.add_toggled(&style);
        assert_eq!(
            sheet.to_css(),
            "@layer hypersig-tokens,hypersig-atoms,hypersig-toggled;\
             @layer hypersig-toggled.k--00{.when\\:color_red{color:red}}"
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
