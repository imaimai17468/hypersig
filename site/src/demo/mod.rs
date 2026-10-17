//! The live demos, each a page of its own that an article shows in an
//! `<iframe>` beside its source.
//!
//! A demo runs on signals alone, since the site is served as static files
//! and has no server to answer an action.

use hypersig::Page;
use hypersig::Signals;
use hypersig::html::{div, el, p};
use hypersig::style::value::{BorderBottomStyle, BorderStyle, FontWeight, px, rem};
use hypersig::style::{Element, Global, Style};
use hypersig_ui::token;

use crate::Site;
use crate::code::{self, Lang};
use crate::lang::L;
use crate::prose::{Part, parts};

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
                    $(Self::$variant => frame_page(site, $module::page),)*
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
    Theme => theme, "ui/theme.rs", "21rem";
    Button => button, "ui/button.rs", "12rem";
    TextField => text_field, "ui/text_field.rs", "20rem";
    Checkbox => checkbox, "ui/checkbox.rs", "9.5rem";
    RadioGroup => radio_group, "ui/radio_group.rs", "13.5rem";
    Select => select, "ui/select.rs", "16rem";
    Slider => slider, "ui/slider.rs", "7rem";
    Listbox => listbox, "ui/listbox.rs", "10.5rem";
    Combobox => combobox, "ui/combobox.rs", "22rem";
    Tabs => tabs, "ui/tabs.rs", "8rem";
    Accordion => accordion, "ui/accordion.rs", "19rem";
    Dialog => dialog, "ui/dialog.rs", "24rem";
    Menu => menu, "ui/menu.rs", "17rem";
    Tooltip => tooltip, "ui/tooltip.rs", "8rem";
    Toast => toast, "ui/toast.rs", "20rem";
    Card => card, "ui/card.rs", "19rem";
    Alert => alert, "ui/alert.rs", "16rem";
    Badge => badge, "ui/badge.rs", "8rem";
    Progress => progress, "ui/progress.rs", "10.5rem";
    Separator => separator, "ui/separator.rs", "12.5rem";
    Hover => hover, "style/hover.rs", "12rem";
    Accent => accent, "style/accent.rs", "15rem";
    Layout => layout, "style/layout.rs", "14rem";
    Spinner => spinner, "style/spinner.rs", "6rem";
}

impl Demo {
    /// The demo's directory inside the site, ending in `/`.
    pub fn path(self) -> String {
        format!("demo/{}/", self.name())
    }
}

/// The function that builds a demo's page: `fn page() -> Page<S>` for a demo
/// whose texts are the same in every language, or `fn page(site: Site) ->
/// Page<S>` for one that writes its texts in the site's language.
///
/// `M` tells the two implementations apart, since a function item implements
/// only one of them.
trait DemoPage<M> {
    type Signals: Signals;

    fn page(self, site: Site) -> Page<Self::Signals>;
}

impl<S: Signals, F: FnOnce() -> Page<S>> DemoPage<fn() -> S> for F {
    type Signals = S;

    fn page(self, _site: Site) -> Page<S> {
        self()
    }
}

impl<S: Signals, F: FnOnce(Site) -> Page<S>> DemoPage<fn(Site) -> S> for F {
    type Signals = S;

    fn page(self, site: Site) -> Page<S> {
        self(site)
    }
}

/// The page `page` builds, with the components' theme and room around its content.
fn frame_page<M>(site: Site, page: impl DemoPage<M>) -> String {
    page.page(site)
        .runtime_src(site.runtime())
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
                    .overflow_x(hypersig::style::value::OverflowX::Hidden),
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
                el("iframe")
                    .attr("src", site.page_url(&demo.path()))
                    .attr(
                        "title",
                        site.t(L {
                            en: format!("The {} demo, running", demo.name().replace('_', " ")),
                            ja: format!("{} のデモの実行結果", demo.name().replace('_', " ")),
                        }),
                    )
                    .attr("loading", "lazy")
                    .css(
                        Style::new()
                            .display(hypersig::style::value::Display::Block)
                            .width(hypersig::style::value::pct(100))
                            .height(hypersig::style::value::raw(demo.height()))
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
/// replaced by the value for `lang`, and the lines that only serve the
/// translation removed, so the code reads as an application would write it.
fn shown_source(source: &str, lang: crate::Lang) -> String {
    const PICK: &str = "site.t(L {";
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(at) = rest.find(PICK) {
        out.push_str(&rest[..at]);
        let inside = &rest[at + PICK.len()..];
        let end = closing(inside, '}').expect("a `site.t(L {` is closed");
        let fields = &inside[..end];
        let value = field(fields, "en:")
            .zip(field(fields, "ja:"))
            .map(|(en, ja)| if lang == crate::Lang::Ja { ja } else { en })
            .expect("an `L` has `en` and `ja`");
        out.push_str(value.trim());
        rest = inside[end + 1..]
            .strip_prefix(')')
            .expect("`site.t(L { .. })` ends with `)`");
    }
    out.push_str(rest);
    out.lines()
        .filter(|line| !matches!(line.trim(), "use crate::Site;" | "use crate::lang::L;"))
        .map(|line| line.replace("(site: Site)", "()"))
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n\n\n", "\n\n")
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
}
