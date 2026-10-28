//! [`Sheet`], which collects styles and writes their rules.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::condition::Conditions;
use crate::style::{Prop, Style, class_name};

/// The cascade layer of the classes from [`Sheet::add`].
pub const LAYER: &str = "hypersig-atoms";

/// The cascade layer of the classes from [`Sheet::add_toggled`].
///
/// It comes after [`LAYER`], so a toggled declaration overrides an element's
/// own.
pub const TOGGLED_LAYER: &str = "hypersig-toggled";

/// The cascade layer of a [`Theme`](crate::Theme)'s values, before [`LAYER`].
pub const TOKEN_LAYER: &str = "hypersig-tokens";

/// The cascade layer of a [`Global`](crate::Global)'s rules.
///
/// It comes after [`TOKEN_LAYER`] and before [`LAYER`], so an element's classes
/// override them.
pub const BASE_LAYER: &str = "hypersig-base";

/// The statement that puts [`TOKEN_LAYER`], [`BASE_LAYER`], [`LAYER`], and
/// [`TOGGLED_LAYER`] in that order. Every bucket, theme, and global
/// stylesheet starts with it, because the first stylesheet to name a layer
/// fixes its place, and any of them may be the first one a page has.
pub(crate) const ORDER: &str =
    "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;";

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
///         "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;\
///          @layer hypersig-atoms.k-----00{.color_crimson{color:crimson}.padding_8px{padding:8px}}",
///         "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;\
///          @layer hypersig-atoms.k----4019-00{.hover\\:color_red:hover{color:red}}",
///     ]
/// );
/// ```
#[derive(Debug, Clone, Default)]
pub struct Sheet {
    rules: BTreeSet<Key>,
    keyframes: BTreeMap<&'static str, String>,
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

/// The rules of one place in the cascade order.
///
/// A place is one cascade layer, one set of conditions, and one shorthand rank,
/// or one `@keyframes` rule.
///
/// Rules in one bucket never set the same property on the same element, so
/// their order among themselves does not matter. A page writes each bucket as
/// its own `<style>` element, so the runtime can add the rules a patch brings
/// to the right bucket, or insert a new bucket at its place in the order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    sort_key: String,
    css: String,
}

impl Bucket {
    /// A string that sorts the way buckets are ordered.
    ///
    /// It is `0` for [`LAYER`] or `1` for [`TOGGLED_LAYER`], a space, and the
    /// sublayer name, whose fixed fields compare as strings the way the buckets
    /// do. A `@keyframes` rule, which no order affects, is `2`, a space, and
    /// its name.
    #[must_use]
    pub fn sort_key(&self) -> &str {
        &self.sort_key
    }

    /// The bucket's rules as a stylesheet of their own.
    ///
    /// It holds the order of the layers, then the rules inside `@layer
    /// <layer>.<sublayer>{...}`.
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
        for keyframes in style.keyframes_used() {
            self.keyframes
                .entry(keyframes.name())
                .or_insert_with(|| keyframes.to_css());
        }
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
        for (name, css) in &other.keyframes {
            self.keyframes.entry(name).or_insert_with(|| css.clone());
        }
    }

    /// Whether no style has been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.keyframes.is_empty()
    }

    /// The rules grouped into buckets, in the order they apply.
    ///
    /// One bucket per `@keyframes` rule follows them.
    pub fn buckets(&self) -> impl Iterator<Item = Bucket> + '_ {
        let mut rules = self.rules.iter().peekable();
        let atoms = std::iter::from_fn(move || {
            let first = rules.next()?;
            let layer = if first.toggled { TOGGLED_LAYER } else { LAYER };
            let key = first.bucket();
            let mut css = format!("{ORDER}@layer {layer}.{key}{{");
            let open = first.conditions.open(&mut css);
            first.write_rule(&mut css);
            while let Some(next) = rules.next_if(|k| k.same_bucket(first)) {
                next.write_rule(&mut css);
            }
            Conditions::close(open, &mut css);
            css.push('}');
            Some(Bucket {
                sort_key: format!("{} {key}", u8::from(first.toggled)),
                css,
            })
        });
        atoms.chain(self.keyframes.iter().map(|(name, css)| Bucket {
            sort_key: format!("2 {name}"),
            css: format!("{ORDER}{css}"),
        }))
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

    /// The sublayer name: `k`, then the media, container, and pseudo-class
    /// conditions, the pseudo-element, and the shorthand rank, each in fixed
    /// width, so that comparing two names as strings compares the buckets.
    /// Each list of conditions ends with `-`, which sorts before the digits,
    /// so a list sorts before any longer list it starts.
    fn bucket(&self) -> String {
        let mut key = String::from("k");
        for &m in &self.conditions.media {
            let _ = write!(key, "{:02}", m as u8);
        }
        key.push('-');
        for &c in &self.conditions.container {
            let _ = write!(key, "{:02}", c as u8);
        }
        key.push('-');
        for &(rank, p) in &self.conditions.group {
            let _ = write!(key, "{rank}{:03}", p as u16);
        }
        key.push('-');
        for &(rank, p) in &self.conditions.peer {
            let _ = write!(key, "{rank}{:03}", p as u16);
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
        let name = class_name(&self.conditions, self.property, &self.value, self.toggled);
        self.conditions.write_selector(out, |out| {
            out.push('.');
            escape_class(&name, out);
        });
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
    use crate::{Condition, Container, Media, PseudoClass, PseudoElement, Style};

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
        let keys: Vec<String> = sheet.buckets().map(|b| b.sort_key().to_owned()).collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
        assert_eq!(keys.len(), 6);
    }

    #[test]
    fn writes_container_queries_inside_media_queries() {
        let mut sheet = Sheet::new();
        sheet.add(
            &Style::new().width(pct(100)).on(
                Container::Xl3,
                Style::new()
                    .width(pct(50))
                    .on(Media::Print, Style::new().width(pct(25))),
            ),
        );
        let css = css(&sheet);
        assert_eq!(css.len(), 3);
        assert!(
            css[1].ends_with(r"{@container (min-width: 48rem){.\@3xl\:width_50\%{width:50%}}}")
        );
        assert!(css[2].ends_with(
            r"{@media print{@container (min-width: 48rem){.print\:\@3xl\:width_25\%{width:25%}}}}"
        ));
    }

    #[test]
    fn writes_print_before_the_features_it_nests_in() {
        let mut sheet = Sheet::new();
        sheet.add(&Style::new().on(
            Media::Md,
            Style::new().on(Media::Print, Style::new().color(Color::RED)),
        ));
        assert!(
            css(&sheet)[0].ends_with(
                r"{@media print and (min-width: 48rem){.md\:print\:color_red{color:red}}}"
            )
        );
    }

    #[test]
    fn writes_group_and_peer_conditions_before_the_class() {
        let style = Style::new()
            .on(
                Condition::Group(PseudoClass::Hover),
                Style::new().display(Display::Block),
            )
            .on(
                Condition::Peer(PseudoClass::Checked),
                Style::new().color(Color::RED),
            );
        assert_eq!(
            style.class_names().collect::<Vec<_>>(),
            ["group-hover:display_block", "peer-checked:color_red"]
        );
        let mut sheet = Sheet::new();
        sheet.add(&style);
        let css = sheet.to_css();
        assert!(css.contains("{.group:hover .group-hover\\:display_block{display:block}}"));
        assert!(css.contains("{.peer:checked ~ .peer-checked\\:color_red{color:red}}"));
        assert_eq!(
            Style::new()
                .group()
                .peer()
                .class_names()
                .collect::<Vec<_>>(),
            ["group", "peer"]
        );
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
            "@layer hypersig-tokens,hypersig-base,hypersig-atoms,hypersig-toggled;\
             @layer hypersig-toggled.k-----00{.when\\:color_red{color:red}}"
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
    fn escapes_the_parentheses_of_the_select_picker() {
        let style =
            Style::new().pseudo(PseudoElement::PickerSelect, Style::new().color(Color::RED));
        let mut sheet = Sheet::new();
        sheet.add(&style);
        assert!(
            sheet
                .to_css()
                .contains(".picker\\(select\\)\\:color_red::picker(select){color:red}")
        );
    }

    #[test]
    fn a_later_value_replaces_an_earlier_one() {
        let style = Style::new()
            .color(Color::RED)
            .merge(Style::new().color(Color::BLUE));
        assert_eq!(style.class_names().collect::<Vec<_>>(), ["color_blue"]);
    }
}
