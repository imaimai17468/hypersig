//! The building blocks of an article: headings, paragraphs, lists, tables, and
//! notes, styled after the shadcn/ui documentation.
//!
//! Text passed to these functions may hold three inline marks. `` `code` ``
//! becomes `<code>`, `**words**` becomes `<strong>`, and `[label](target)`
//! becomes a link. A target is a path inside the site such as
//! `learn/signals/`, a rustdoc page under `api:` such as
//! `api:hypersig/struct.Page.html`, or a full `https://` URL. The links
//! inside the site are checked by `check_links`.

use std::iter;

use hypersig::html::{
    El, Id, Node, Role, a, code, div, h1, h2, h3, li, p, strong, table, td, th, tr, ul,
};
use hypersig::style::value::{
    BorderBottomStyle, BorderCollapse, BorderStyle, FontWeight, LineHeight, OverflowWrap,
    OverflowX, TextAlign, TextDecorationLine, VerticalAlign, pct, px, raw, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{SafeUrl, Signals, html};
use hypersig_ui::token;
use serde::{Deserialize, Serialize};

use crate::lang::Text;
use crate::{Site, own};

/// The signals of a documentation page.
#[derive(Signals, Serialize, Deserialize, Default)]
pub struct Docs {
    /// Whether the sidebar is open on a narrow screen.
    pub menu: bool,
    /// The color scheme the visitor picked with the button in the header,
    /// which `Page::persist` keeps from one page to the next.
    pub scheme: Scheme,
}

/// A color scheme of the site.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Scheme {
    /// The scheme the system prefers, through `prefers-color-scheme`.
    #[default]
    System,
    Light,
    Dark,
}

/// An element of a documentation page.
pub type Part = El<Docs>;

/// The page title.
pub fn title(text: &str) -> Part {
    h1().css(
        Style::new()
            .font_size(rem(2.25))
            .line_height(LineHeight::Length(rem(2.5)))
            .font_weight(FontWeight::Number(700.0))
            .letter_spacing(rem(-0.025)),
    )
    .text(text)
}

/// The paragraph under the title that says what the page covers.
pub fn lead<'a>(site: Site, text: impl Into<Text<'a, str>>) -> Part {
    let text = site.text(text);
    p().css(
        Style::new()
            .margin_top(rem(0.75))
            .font_size(rem(1.125))
            .line_height(LineHeight::Length(rem(1.75)))
            .color(token::muted_foreground().var()),
    )
    .children(inline(site, text))
}

/// A section heading, with an id made from its English text so it can be
/// linked to, and so the same section has the same id in every language.
pub fn heading<'a>(site: Site, text: impl Into<Text<'a, str>>) -> Part {
    let text = text.into();
    h2().id(Id::from_text(anchor(text.english())))
        .css(
            Style::new()
                .margin_top(rem(2.5))
                .padding_bottom(rem(0.5))
                .border_bottom_width(px(1))
                .border_bottom_style(BorderBottomStyle::Solid)
                .border_bottom_color(token::border().var())
                .font_size(rem(1.5))
                .line_height(LineHeight::Length(rem(2)))
                .font_weight(FontWeight::Number(600.0))
                .letter_spacing(rem(-0.0125)),
        )
        .text(text.get(site.lang()))
}

/// A heading inside a section, with an id made from its English text.
pub fn subheading<'a>(site: Site, text: impl Into<Text<'a, str>>) -> Part {
    let text = text.into();
    h3().id(Id::from_text(anchor(text.english())))
        .css(
            Style::new()
                .margin_top(rem(2))
                .font_size(rem(1.25))
                .line_height(LineHeight::Length(rem(1.75)))
                .font_weight(FontWeight::Number(600.0)),
        )
        .text(text.get(site.lang()))
}

/// A paragraph.
pub fn para<'a>(site: Site, text: impl Into<Text<'a, str>>) -> Part {
    let text = site.text(text);
    p().css(body_text().margin_top(rem(1.25)))
        .children(inline(site, text))
}

/// A bulleted list, one item per entry of `items`.
pub fn bullets<'a, I: Into<Text<'a, str>> + Copy>(site: Site, items: &[I]) -> Part {
    ul().css(
        body_text()
            .margin_top(rem(1.25))
            .padding_left(rem(1.5))
            .list_style_type(raw("disc")),
    )
    .children(items.iter().map(|&item| {
        li().css(Style::new().margin_top(rem(0.5)))
            .children(inline(site, site.text(item)))
    }))
}

/// A table with a header row and one row per entry of `rows`.
pub fn table_of<'a, R: AsRef<[&'a str]> + 'a>(
    site: Site,
    header: impl Into<Text<'a, [&'a str]>>,
    rows: impl Into<Text<'a, [R]>>,
) -> Part {
    let header = site.text(header);
    let rows = site.text(rows);
    let cell = || {
        Style::new()
            .padding_block(rem(0.5))
            .padding_inline(rem(0.75))
            .border_width(px(1))
            .border_style(BorderStyle::Solid)
            .border_color(token::border().var())
            .text_align(TextAlign::Left)
            .vertical_align(VerticalAlign::Top)
    };
    div()
        .css(
            Style::new()
                .margin_top(rem(1.5))
                .width(pct(100))
                .overflow_x(OverflowX::Auto),
        )
        .child(
            table()
                .css(
                    Style::new()
                        .width(pct(100))
                        .border_collapse(BorderCollapse::Collapse)
                        .font_size(rem(0.875))
                        .line_height(LineHeight::Length(rem(1.5))),
                )
                .child(tr().children(header.iter().map(|text| {
                    th().css(
                        cell()
                            .font_weight(FontWeight::Number(600.0))
                            .background_color(token::muted().var()),
                    )
                    .children(inline(site, text))
                })))
                .children(rows.iter().map(|row| {
                    tr().children(
                        row.as_ref()
                            .iter()
                            .map(|text| td().css(cell()).children(inline(site, text))),
                    )
                })),
        )
}

/// A boxed note beside the text, for a fact the reader should not miss.
pub fn note<'a>(site: Site, text: impl Into<Text<'a, str>>) -> Part {
    let text = site.text(text);
    div()
        .role(Role::Note)
        .css(
            body_text()
                .margin_top(rem(1.5))
                .padding_block(rem(0.75))
                .padding_inline(rem(1))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(token::radius().var())
                .background_color(token::muted().var()),
        )
        .children(inline(site, text))
}

/// The style of running text.
fn body_text() -> Style {
    Style::new().line_height(LineHeight::Number(1.75))
}

/// The id of a heading: its text in lowercase, with each run of characters
/// other than ASCII letters and digits replaced by one `-`.
pub fn anchor(text: &str) -> String {
    let mut id = String::new();
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            id.push(c.to_ascii_lowercase());
        } else if !id.is_empty() && !id.ends_with('-') {
            id.push('-');
        }
    }
    id.trim_end_matches('-').to_owned()
}

/// One piece of a text with inline marks: plain text, or a mark with the
/// text inside it.
enum Mark<'a> {
    Text(&'a str),
    Code(&'a str),
    Bold(&'a str),
    Link { label: &'a str, target: &'a str },
}

/// The pieces of `text`, in order. A `` ` ``, `*`, or `[` that opens no mark
/// is a [`Mark::Text`] of its own.
///
/// # Panics
///
/// Panics if a code or bold mark is not closed, which is a typo in the
/// page's source.
fn marks(text: &str) -> impl Iterator<Item = Mark<'_>> {
    let mut rest = text;
    iter::from_fn(move || {
        if rest.is_empty() {
            return None;
        }
        let at = rest.find(['`', '*', '[']).unwrap_or(rest.len());
        if at > 0 {
            let (before, mark) = rest.split_at(at);
            rest = mark;
            return Some(Mark::Text(before));
        }
        let mark = rest;
        Some(if let Some(after) = mark.strip_prefix('`') {
            let end = after.find('`').expect("a code mark is closed");
            rest = &after[end + 1..];
            Mark::Code(&after[..end])
        } else if let Some(after) = mark.strip_prefix("**") {
            let end = after.find("**").expect("a bold mark is closed");
            rest = &after[end + 2..];
            Mark::Bold(&after[..end])
        } else if let Some((label, target, after)) = split_link(mark) {
            rest = after;
            Mark::Link { label, target }
        } else {
            rest = &mark[1..];
            Mark::Text(&mark[..1])
        })
    })
}

/// `text` with its inline marks turned into elements.
///
/// # Panics
///
/// Panics if a mark is not closed, which is a typo in the page's source.
pub fn inline(site: Site, text: &str) -> Vec<Node<Docs>> {
    marks(text)
        .map(|mark| match mark {
            Mark::Text(text) => text.into(),
            Mark::Code(text) => inline_code(text).into(),
            Mark::Bold(text) => strong()
                .css(Style::new().font_weight(FontWeight::Number(600.0)))
                .text(text)
                .into(),
            Mark::Link { label, target } => link(site, target).children(inline(site, label)).into(),
        })
        .collect()
}

/// `text` with the marks [`inline`] reads taken out, for a place that shows
/// no markup, such as a page's description.
pub fn plain(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for mark in marks(text) {
        match mark {
            Mark::Text(text) | Mark::Code(text) | Mark::Bold(text) => out.push_str(text),
            Mark::Link { label, .. } => out.push_str(&plain(label)),
        }
    }
    out
}

/// The label, the target, and the text after a link at the start of `text`,
/// or `None` where `text` does not start with one, as in `[i32]`.
fn split_link(text: &str) -> Option<(&str, &str, &str)> {
    let after = text.strip_prefix('[')?;
    let label_end = after.find("](")?;
    if after[..label_end].contains('[') {
        return None;
    }
    let target_start = label_end + 2;
    let target_end = after[target_start..].find(')')? + target_start;
    Some((
        &after[..label_end],
        &after[target_start..target_end],
        &after[target_end + 1..],
    ))
}

/// Code inside running text.
pub fn inline_code(text: &str) -> Part {
    code()
        .css(
            Style::new()
                .padding_block(rem(0.125))
                .padding_inline(rem(0.3))
                .border_radius(px(4))
                .background_color(token::muted().var())
                .font_size(pct(87.5))
                .overflow_wrap(OverflowWrap::BreakWord),
        )
        .text(text)
}

/// A link to `target`, in the form the module header describes.
pub fn link(site: Site, target: &str) -> Part {
    a().href(href(site, target)).css(
        Style::new()
            .font_weight(FontWeight::Number(500.0))
            .text_decoration_line(TextDecorationLine::Underline)
            .text_underline_offset(px(4))
            .on(
                PseudoClass::Hover,
                Style::new().color(token::muted_foreground().var()),
            ),
    )
}

/// The URL of `target`, in the form the module header describes.
pub fn href(site: Site, target: &str) -> SafeUrl {
    own(if target.starts_with("https://") {
        target.to_owned()
    } else if let Some(item) = target.strip_prefix("api:") {
        site.url(&format!("{}/{item}", crate::API_DIR))
    } else {
        site.page_url(target)
    })
}

/// Several parts as one, for a function that returns the body of a page.
pub fn parts(children: impl IntoIterator<Item = Part>) -> Part {
    html::div().children(children)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_marks_become_elements() {
        let site = Site::new("/hypersig");
        let html = para(
            site,
            Text::Chosen("Call `Sig::set`, **then** read [Signals](learn/signals/)."),
        )
        .render();
        assert!(html.contains(">Sig::set</code>"), "{html}");
        assert!(html.contains(">then</strong>"), "{html}");
        assert!(
            html.contains(r#"href="/hypersig/learn/signals/""#),
            "{html}"
        );
    }

    #[test]
    fn a_bracket_without_a_target_stays_text() {
        let site = Site::new("");
        let html = para(site, Text::Chosen("A `Vec` reads as [T] here.")).render();
        assert!(html.contains("as [T] here."), "{html}");
    }

    #[test]
    fn a_lone_star_stays_text() {
        let site = Site::new("");
        let html = para(site, Text::Chosen("2 * 3")).render();
        assert!(html.contains(">2 * 3</p>"), "{html}");
    }

    #[test]
    fn an_anchor_keeps_letters_and_digits() {
        assert_eq!(
            anchor("Talking to the server, step 2"),
            "talking-to-the-server-step-2"
        );
    }
}
