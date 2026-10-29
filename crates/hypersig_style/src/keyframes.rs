//! [`Keyframes`], an `@keyframes` rule that a [`Style`] animates with.

use std::fmt::Write as _;

use crate::condition::Conditions;
use crate::style::Style;

/// The steps of an animation, written as `@keyframes <name>`.
///
/// [`Style::keyframes`] sets an element's `animation-name` to it and carries
/// the rule along, so the page or the patch that brings the element also
/// brings the rule.
///
/// Two `Keyframes` with the same name are one rule to the browser, so the
/// styles and the [`Sheet`](crate::Sheet) that meet both panic unless their steps are
/// the same. The check covers a page's [`Global`](crate::Global) rules and
/// body styles together, and one patch. A later patch whose
/// keyframes reuse a name of the page with other steps is not compared,
/// and the browser plays the steps the patch brought.
///
/// ```
/// use hypersig_style::value::{AnimationIterationCount, Transform, deg, ms, raw};
/// use hypersig_style::{Keyframes, Sheet, Style};
///
/// let spin = Keyframes::new("spin")
///     .at(0, Style::new().transform(raw("rotate(0deg)")))
///     .at(100, Style::new().transform(raw("rotate(360deg)")));
/// let spinner = Style::new()
///     .keyframes(&spin)
///     .animation_duration(ms(800))
///     .animation_iteration_count(AnimationIterationCount::Infinite);
/// let mut sheet = Sheet::new();
/// sheet.add(&spinner);
/// assert!(sheet.to_css().contains("@keyframes spin{0%{transform:rotate(0deg)}100%{transform:rotate(360deg)}}"));
/// # let _ = (Transform::None, deg(0));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframes {
    name: &'static str,
    steps: Vec<(u8, Style)>,
}

impl Keyframes {
    /// Keyframes named `name`, with no steps yet.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty, starts with a digit, or holds a character
    /// other than an ASCII letter, digit, `-`, or `_`, which an
    /// `animation-name` would need quoted.
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        assert!(
            name.bytes().next().is_some_and(|b| !b.is_ascii_digit())
                && name
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
            "a keyframes name is an identifier of ASCII letters, digits, `-`, and `_`, not {name:?}"
        );
        Self {
            name,
            steps: Vec::new(),
        }
    }

    /// Adds the declarations of `style` at `percent` of the animation.
    ///
    /// Only the unconditioned declarations of `style` are written, because a
    /// keyframe takes no selector or media query.
    ///
    /// # Panics
    ///
    /// Panics if `percent` is over 100.
    #[must_use]
    pub fn at(mut self, percent: u8, style: Style) -> Self {
        assert!(
            percent <= 100,
            "a keyframe is at 0 to 100 percent, not {percent}"
        );
        self.steps.push((percent, style));
        self
    }

    /// The name `animation-name` refers to.
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Panics unless `css` and `other`, two `@keyframes` rules named `name`,
    /// are the same rule.
    pub(crate) fn assert_one_rule(name: &str, css: &str, other: &str) {
        assert!(
            css == other,
            "two Keyframes named {name:?} have different steps, `{css}` and `{other}`; give each its own name"
        );
    }

    pub(crate) fn to_css(&self) -> String {
        let mut out = format!("@keyframes {}{{", self.name);
        for (percent, style) in &self.steps {
            let _ = write!(out, "{percent}%{{");
            let mut first = true;
            for atom in style
                .atoms()
                .iter()
                .filter(|a| a.conditions == Conditions::default())
            {
                if !first {
                    out.push(';');
                }
                first = false;
                atom.property.write_name(&mut out);
                let _ = write!(out, ":{}", atom.value);
            }
            out.push('}');
        }
        out.push('}');
        out
    }
}
