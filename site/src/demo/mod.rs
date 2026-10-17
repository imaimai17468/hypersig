//! The live demos, each a page of its own that an article shows in an
//! `<iframe>` beside its source.
//!
//! A demo runs on signals alone, since the site is served as static files
//! and has no server to answer an action.

mod cart;
mod counter;
mod greeting;
mod status;
mod stopwatch;
mod temperature;
mod todos;

use hypersig::Page;
use hypersig::Signals;
use hypersig::html::{div, el, p};
use hypersig::style::value::{BorderBottomStyle, BorderStyle, FontWeight, px, rem};
use hypersig::style::{Element, Global, Style};
use hypersig_ui::token;

use crate::Site;
use crate::code::{self, Lang};
use crate::prose::{Part, parts};

/// One live demo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Demo {
    Counter,
    Greeting,
    Cart,
    Temperature,
    Todos,
    Status,
    Stopwatch,
}

impl Demo {
    /// Every demo.
    pub const ALL: &[Self] = &[
        Self::Counter,
        Self::Greeting,
        Self::Cart,
        Self::Temperature,
        Self::Todos,
        Self::Status,
        Self::Stopwatch,
    ];

    /// The demo's directory inside the site, ending in `/`.
    pub fn path(self) -> String {
        format!("demo/{}/", self.name())
    }

    fn name(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Greeting => "greeting",
            Self::Cart => "cart",
            Self::Temperature => "temperature",
            Self::Todos => "todos",
            Self::Status => "status",
            Self::Stopwatch => "stopwatch",
        }
    }

    /// The demo's source, as the article shows it.
    fn source(self) -> &'static str {
        code::without_header(match self {
            Self::Counter => include_str!("counter.rs"),
            Self::Greeting => include_str!("greeting.rs"),
            Self::Cart => include_str!("cart.rs"),
            Self::Temperature => include_str!("temperature.rs"),
            Self::Todos => include_str!("todos.rs"),
            Self::Status => include_str!("status.rs"),
            Self::Stopwatch => include_str!("stopwatch.rs"),
        })
    }

    /// The height of the frame, enough for the demo without a scrollbar.
    fn height(self) -> &'static str {
        match self {
            Self::Counter | Self::Stopwatch => "6rem",
            Self::Greeting => "11rem",
            Self::Cart | Self::Temperature => "10rem",
            Self::Todos => "19rem",
            Self::Status => "7rem",
        }
    }

    /// The demo's page as an HTML document.
    pub fn render(self, site: Site) -> String {
        match self {
            Self::Counter => frame_page(site, counter::page()),
            Self::Greeting => frame_page(site, greeting::page()),
            Self::Cart => frame_page(site, cart::page()),
            Self::Temperature => frame_page(site, temperature::page()),
            Self::Todos => frame_page(site, todos::page()),
            Self::Status => frame_page(site, status::page()),
            Self::Stopwatch => frame_page(site, stopwatch::page()),
        }
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

/// The source of `demo` above the demo running in a frame.
pub fn show(site: Site, demo: Demo) -> Part {
    parts([
        code::block(
            Lang::Rust,
            Some(&format!("{}.rs", demo.name())),
            demo.source(),
        ),
        div()
            .css(
                Style::new()
                    .margin_top(rem(0.75))
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
                    .attr("title", format!("The {} demo, running", demo.name()))
                    .attr("loading", "lazy")
                    .css(
                        Style::new()
                            .display(hypersig::style::value::Display::Block)
                            .width(hypersig::style::value::pct(100))
                            .height(hypersig::style::value::raw(demo.height()))
                            .border_width(px(0)),
                    ),
            ),
    ])
}
