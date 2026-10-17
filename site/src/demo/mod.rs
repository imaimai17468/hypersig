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
                    $(Self::$variant => frame_page(site, $module::page()),)*
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

/// `page` with the components' theme and room around its content.
fn frame_page<S: Signals>(site: Site, page: Page<S>) -> String {
    page.runtime_src(site.runtime())
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
                .text("Result"),
            )
            .child(
                el("iframe")
                    .attr("src", site.url(&demo.path()))
                    .attr(
                        "title",
                        format!("The {} demo, running", demo.name().replace('_', " ")),
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
            demo.source(),
        ),
    ])
}
