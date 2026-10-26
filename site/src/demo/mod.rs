//! The live demos, each a page of its own that an article shows in an
//! `<iframe>` beside its source.
//!
//! A demo runs on signals alone, since the site is served as static files
//! and has no server to answer an action.

use std::cell::Cell;

use hypersig::Page;
use hypersig::Signals;
use hypersig::html::{attr, div, el, p};
use hypersig::style::value::{
    BorderBottomStyle, BorderStyle, Display, FontWeight, OverflowX, pct, px, raw, rem,
};
use hypersig::style::{Element, Global, Style};
use hypersig_ui::token;

use crate::code::{self, Lang};
use crate::lang::L;
use crate::prose::{Part, parts};
use crate::{Site, own};

/// Declares each demo: its variant, its module and file, and the height of
/// its frame, enough for the demo without a scrollbar.
macro_rules! demos {
    ($($variant:ident => $module:ident, $file:literal, $height:literal;)*) => {
        $(
            #[path = $file]
            mod $module;
        )*

        /// One live demo.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Demo {
            $($variant,)*
        }

        impl Demo {
            /// Every demo.
            pub const ALL: &[Self] = &[$(Self::$variant,)*];

            /// The demo's name: its file name without the directory and
            /// the extension.
            fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($module),)*
                }
            }

            /// The demo's source, as the article shows it.
            fn source(self) -> &'static str {
                code::without_header(match self {
                    $(Self::$variant => include_str!($file),)*
                })
            }

            fn height(self) -> &'static str {
                match self {
                    $(Self::$variant => $height,)*
                }
            }

            /// The demo's page as an HTML document.
            pub fn render(self, site: Site) -> String {
                match self {
                    $(Self::$variant => rendering(site, || frame_page(site, $module::page(site))),)*
                }
            }
        }
    };
}

demos! {
    Counter => counter, "counter.rs", "6rem";
    Greeting => greeting, "greeting.rs", "11rem";
    Cart => cart, "cart.rs", "10rem";
    Temperature => temperature, "temperature.rs", "10.5rem";
    Todos => todos, "todos.rs", "19rem";
    Status => status, "status.rs", "8rem";
    Stopwatch => stopwatch, "stopwatch.rs", "6rem";
    Rating => rating, "rating.rs", "6rem";
    Signup => signup, "signup.rs", "38.5rem";
    Theme => theme, "ui/theme.rs", "21rem";
    Button => button, "ui/button.rs", "12rem";
    TextField => text_field, "ui/text_field.rs", "20rem";
    Checkbox => checkbox, "ui/checkbox.rs", "9.5rem";
    RadioGroup => radio_group, "ui/radio_group.rs", "13.5rem";
    Select => select, "ui/select.rs", "16rem";
    Slider => slider, "ui/slider.rs", "7rem";
    Toggle => toggle, "ui/toggle.rs", "8rem";
    Listbox => listbox, "ui/listbox.rs", "10.5rem";
    Combobox => combobox, "ui/combobox.rs", "22rem";
    Tabs => tabs, "ui/tabs.rs", "8rem";
    Accordion => accordion, "ui/accordion.rs", "19rem";
    Collapsible => collapsible, "ui/collapsible.rs", "10rem";
    Dialog => dialog, "ui/dialog.rs", "24rem";
    Menu => menu, "ui/menu.rs", "17rem";
    Tooltip => tooltip, "ui/tooltip.rs", "8rem";
    Popover => popover, "ui/popover.rs", "18rem";
    Toast => toast, "ui/toast.rs", "20rem";
    Card => card, "ui/card.rs", "19rem";
    Alert => alert, "ui/alert.rs", "16rem";
    Badge => badge, "ui/badge.rs", "8rem";
    Progress => progress, "ui/progress.rs", "10.5rem";
    Avatar => avatar, "ui/avatar.rs", "5rem";
    Separator => separator, "ui/separator.rs", "12.5rem";
    Navigation => navigation, "ui/navigation.rs", "15rem";
    DataTable => data_table, "ui/data_table.rs", "25rem";
    Chart => chart, "ui/chart.rs", "29rem";
    Hover => hover, "style/hover.rs", "12rem";
    Accent => accent, "style/accent.rs", "15rem";
    Layout => layout, "style/layout.rs", "14rem";
    Spinner => spinner, "style/spinner.rs", "6rem";
}

/// The script of `<star-rating>`, which the rating demo loads and the page
/// on JavaScript libraries shows.
pub const STAR_RATING_JS: &str = include_str!("star-rating.js");

thread_local! {
    /// The site the demo being rendered is for.
    static RENDERING: Cell<Option<Site>> = const { Cell::new(None) };
}

impl Site {
    /// The site the demo being rendered is for, read by the code that takes no
    /// `site` argument: `Validate::rules` gets only the form and its rules,
    /// and its messages are translated like every other text of a demo.
    ///
    /// # Panics
    ///
    /// Panics outside [`Demo::render`].
    fn rendering() -> Self {
        RENDERING
            .get()
            .expect("a demo's rules are built inside `Demo::render`")
    }
}

/// `render()`, with [`Site::rendering`] returning `site` while it runs.
fn rendering(site: Site, render: impl FnOnce() -> String) -> String {
    let outer = RENDERING.replace(Some(site));
    let page = render();
    RENDERING.set(outer);
    page
}

impl Demo {
    /// The demo's directory inside the site, ending in `/`.
    pub fn path(self) -> String {
        format!("demo/{}/", self.name())
    }
}

/// `page` with the components' theme and room around its content.
///
/// A demo is shown inside a page of the docs, so search engines are asked
/// not to list it on its own.
fn frame_page<S: Signals>(site: Site, page: Page<S>) -> String {
    page.runtime_src(site.runtime())
        .icon(site.icon())
        .noindex()
        .global(hypersig_ui::global())
        .global(
            Global::new()
                .rule(Element::Body, Style::new().padding(rem(1.5)))
                .rule(Element::Ul, Style::new().padding_left(px(0))),
        )
        .theme(hypersig_ui::theme())
        .render()
}

/// `demo` running in a frame, above its source.
pub fn show(site: Site, demo: Demo) -> Part {
    parts([
        div()
            .css(
                Style::new()
                    .margin_top(rem(1.5))
                    .border_width(px(1))
                    .border_style(BorderStyle::Solid)
                    .border_color(token::border().var())
                    .border_radius(token::radius().var())
                    .overflow_x(OverflowX::Hidden),
            )
            .child(
                p().css(
                    Style::new()
                        .padding_block(rem(0.5))
                        .padding_inline(rem(1))
                        .border_bottom_width(px(1))
                        .border_bottom_style(BorderBottomStyle::Solid)
                        .border_bottom_color(token::border().var())
                        .font_size(rem(0.75))
                        .font_weight(FontWeight::Number(500.0))
                        .color(token::muted_foreground().var()),
                )
                .text(site.t(L {
                    en: "Result",
                    ja: "実行結果",
                })),
            )
            .child(
                el(hypersig::tag!("iframe"))
                    .src(own(site.page_url(&demo.path())))
                    .attr(
                        attr::TITLE,
                        site.t(L {
                            en: format!("The {} demo, running", demo.name().replace('_', " ")),
                            ja: format!("{} のデモの実行結果", demo.name().replace('_', " ")),
                        }),
                    )
                    .attr(attr::LOADING, "lazy")
                    .css(
                        Style::new()
                            .display(Display::Block)
                            .width(pct(100))
                            .height(raw(demo.height()))
                            .border_width(px(0)),
                    ),
            ),
        code::block(
            Lang::Rust,
            Some(&format!("{}.rs", demo.name())),
            &shown_source(demo.source(), site.lang()),
        ),
    ])
}

/// `source` as the article shows it in `lang`: each `site.t(L { en, ja })`
/// replaced by the value for `lang`, the lines that only serve the
/// translation removed, and the script path this site builds under its base
/// replaced by the path an application serves it at, so the code reads as an
/// application would write it.
fn shown_source(source: &str, lang: crate::Lang) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some((at, inside)) = find_pick(rest) {
        out.push_str(&rest[..at]);
        let inside = &rest[inside..];
        let end = closing(inside, '}').expect("a `site.t(L {` is closed");
        let fields = &inside[..end];
        let value = field(fields, "en:")
            .zip(field(fields, "ja:"))
            .map(|(en, ja)| if lang == crate::Lang::Ja { ja } else { en })
            .expect("an `L` has `en` and `ja`");
        out.push_str(value.trim().trim_end_matches(','));
        let after = inside[end + 1..].trim_start();
        rest = after
            .strip_prefix(',')
            .unwrap_or(after)
            .trim_start()
            .strip_prefix(')')
            .expect("`site.t(L { .. })` ends with `)`");
    }
    out.push_str(rest);
    out.lines()
        .filter(|line| {
            !matches!(
                line.trim(),
                "use crate::Site;" | "use crate::lang::L;" | "let site = Site::rendering();"
            )
        })
        .map(|line| {
            line.replace("(site: Site)", "()").replace(
                "site.star_rating()",
                &format!("\"/{}\"", crate::STAR_RATING_FILE),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n\n\n", "\n\n")
}

/// The start of the first `site.t(L {` in `text`, rustfmt's line breaks
/// between its tokens included, and the index just past its `{`.
fn find_pick(text: &str) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(found) = text[from..].find("site") {
        let start = from + found;
        let mut rest = &text[start + "site".len()..];
        let tokens = [".t(", "L", "{"];
        let matched = tokens.iter().all(|token| {
            rest = rest.trim_start();
            rest.strip_prefix(token).is_some_and(|after| {
                rest = after;
                true
            })
        });
        if matched {
            return Some((start, text.len() - rest.len()));
        }
        from = start + "site".len();
    }
    None
}

/// The expression after `name` in the fields of a struct literal, up to its top-level `,`.
fn field<'a>(fields: &'a str, name: &str) -> Option<&'a str> {
    let start = fields.find(name)? + name.len();
    let value = &fields[start..];
    Some(&value[..closing(value, ',').unwrap_or(value.len())])
}

/// The byte index of the first `stop` in `text` outside brackets and string literals.
fn closing(text: &str, stop: char) -> Option<usize> {
    let mut depth = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    for (i, c) in text.char_indices() {
        if in_string {
            match c {
                _ if escaped => escaped = false,
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            c if c == stop && depth == 0 => return Some(i),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.checked_sub(1)?,
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shown_source_keeps_the_text_of_one_language() {
        let source = "use crate::Site;\nuse crate::lang::L;\n\npub fn page(site: Site) -> Page<S> {\n    p().text(site.t(L {\n        en: \"Hello, {name}\",\n        ja: format!(\"{}、こんにちは\", x),\n    }))\n}\n";
        assert_eq!(
            shown_source(source, crate::Lang::En),
            "\npub fn page() -> Page<S> {\n    p().text(\"Hello, {name}\")\n}"
        );
        assert!(
            shown_source(source, crate::Lang::Ja)
                .contains("p().text(format!(\"{}、こんにちは\", x))")
        );
    }

    #[test]
    fn the_shown_source_names_the_script_by_its_path() {
        let shown = shown_source("    .script(site.star_rating())\n", crate::Lang::En);
        assert_eq!(shown, "    .script(\"/star-rating.js\")");
    }

    #[test]
    fn the_shown_source_finds_a_pick_that_rustfmt_split() {
        let source = "title: site\n    .t(L {\n        en: \"Read\",\n        ja: \"読む\",\n    })\n    .into(),\nb.text(site.t(\n    L {\n        en: \"Add\",\n        ja: \"追加\",\n    },\n));";
        assert_eq!(
            shown_source(source, crate::Lang::En),
            "title: \"Read\"\n    .into(),\nb.text(\"Add\");"
        );
    }
}
