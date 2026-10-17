//! The landing page.

use hypersig::html::{a, div, h1, h2, h3, p, section};
use hypersig::style::pattern::{container, stack, wrap};
use hypersig::style::value::{BorderStyle, FontWeight, LineHeight, TextAlign, raw, rem};
use hypersig::style::{Media, Style};
use hypersig_ui::{ButtonSize, ButtonVariant, button_style, token};

use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn, Reference};
use crate::layout::{self, REPOSITORY};
use crate::prose::{Part, inline, para, table_of};
use crate::{Site, prose};

pub fn render(site: Site) -> String {
    let body = div()
        .css(container(rem(64), rem(1)).padding_bottom(rem(6)))
        .child(hero(site))
        .child(features(site))
        .child(
            section()
                .css(Style::new().margin_top(rem(5)))
                .child(section_title("Write the page in Rust"))
                .child(para(
                    site,
                    "The signal struct holds the page's state. `Counter::count()` is a typed reference to its field, and `count.set(count + 1)` is a statement the browser runs on a click without asking the server. The page renders on the server with the count already in place, so it shows before the runtime loads.",
                ))
                .child(demo::show(site, Demo::Counter)),
        )
        .child(
            section()
                .css(Style::new().margin_top(rem(5)))
                .child(section_title("How it compares"))
                .child(para(
                    site,
                    "Adding a page to a Rust server leaves two choices today. Leptos type-checks what passes between the page and the server, and needs wasm and its build. htmx needs no build, but the URLs and expressions written in its attributes are not type-checked.",
                ))
                .child(table_of(
                    site,
                    &["", "hypersig", "Leptos", "React", "Next.js", "TanStack Start", "Nuxt", "htmx"],
                    &[
                        &["Page written in", "Rust", "Rust", "TypeScript", "TypeScript", "TypeScript", "TypeScript and Vue templates", "HTML attributes"],
                        &["Server written in", "Rust", "Rust", "Any", "TypeScript", "TypeScript", "TypeScript", "Any"],
                        &["No browser build", "Yes", "No (wasm and cargo-leptos)", "No (Vite)", "No (Node bundler)", "No (Vite)", "No (Vite)", "Yes"],
                        &["Request types checked at compile time", "Yes", "Yes", "No", "Yes", "Yes", "No", "No"],
                        &["State kept in the browser", "Yes", "Yes", "Yes", "Yes", "Yes", "Yes", "No"],
                    ],
                )),
        )
        .child(
            section()
                .css(Style::new().margin_top(rem(5)))
                .child(section_title("Benchmarks"))
                .child(para(
                    site,
                    "A 2000-item TodoMVC that stores every add, toggle, and delete on the server. Medians of 3 runs in Chrome 155 on an Apple M4 Max, with the CPU slowed 4x and 50 ms added to every request. The apps and the harness are in [`bench/compare`](https://github.com/imaimai17468/hypersig/tree/main/bench/compare).",
                ))
                .child(table_of(
                    site,
                    &["", "hypersig", "Leptos", "React", "Next.js", "TanStack Start", "Nuxt", "htmx"],
                    &[
                        &["JS (gzip)", "13.4 KB", "140.5 KB", "67.1 KB", "133.3 KB", "119.5 KB", "59.2 KB", "16.5 KB"],
                        &["First Contentful Paint", "179 ms", "190 ms", "184 ms", "198 ms", "188 ms", "172 ms", "235 ms"],
                        &["Total Blocking Time", "0 ms", "27 ms", "2 ms", "5 ms", "0 ms", "25 ms", "233 ms"],
                        &["Interaction to Next Paint", "24 ms", "16 ms", "24 ms", "16 ms", "24 ms", "16 ms", "16 ms"],
                    ],
                )),
        );
    layout::document(site, Doc::Home, body)
}

fn hero(site: Site) -> Part {
    section()
        .css(
            stack(rem(1.5))
                .padding_top(rem(5))
                .padding_bottom(rem(3))
                .text_align(TextAlign::Center)
                .align_items(hypersig::style::value::AlignItems::Center),
        )
        .child(
            h1().css(
                Style::new()
                    .max_width(rem(48))
                    .font_size(rem(2.5))
                    .line_height(LineHeight::Number(1.1))
                    .font_weight(FontWeight::Number(700.0))
                    .letter_spacing(rem(-0.05))
                    .on(Media::Md, Style::new().font_size(rem(3.75))),
            )
            .text("The page, its behavior, and the server, all in Rust"),
        )
        .child(
            p().css(
                Style::new()
                    .max_width(rem(42))
                    .font_size(rem(1.125))
                    .line_height(LineHeight::Length(rem(1.75)))
                    .color(token::muted_foreground().var()),
            )
            .children(inline(
                site,
                "hypersig is a web framework on axum. `cargo build` builds it, with no Node and no wasm, and a mismatch between the page and the server is a compile error.",
            )),
        )
        .child(
            div()
                .css(wrap(rem(0.75)).justify_content(hypersig::style::value::JustifyContent::Center))
                .child(
                    a().attr("href", Doc::Learn(Learn::QuickStart).url(site))
                        .css(button_style(ButtonVariant::Default, ButtonSize::Lg))
                        .text("Get started"),
                )
                .child(
                    a().attr("href", Doc::Reference(Reference::Overview).url(site))
                        .css(button_style(ButtonVariant::Outline, ButtonSize::Lg))
                        .text("API reference"),
                )
                .child(
                    a().attr("href", REPOSITORY)
                        .css(button_style(ButtonVariant::Ghost, ButtonSize::Lg))
                        .text("GitHub"),
                ),
        )
}

fn features(site: Site) -> Part {
    let card = |heading: &str, text: &str| {
        div()
            .css(
                stack(rem(0.5))
                    .padding(rem(1.5))
                    .border_width(hypersig::style::value::px(1))
                    .border_style(BorderStyle::Solid)
                    .border_color(token::border().var())
                    .border_radius(token::radius().var())
                    .background_color(token::card().var()),
            )
            .child(
                h3().css(Style::new().font_weight(FontWeight::Number(600.0)))
                    .text(heading),
            )
            .child(
                p().css(
                    Style::new()
                        .font_size(rem(0.875))
                        .line_height(LineHeight::Length(rem(1.5)))
                        .color(token::muted_foreground().var()),
                )
                .children(inline(site, text)),
            )
    };
    div()
        .css(
            stack(rem(1)).on(
                Media::Md,
                Style::new()
                    .display(hypersig::style::value::Display::Grid)
                    .grid_template_columns(raw("repeat(3, minmax(0, 1fr))")),
            ),
        )
        .child(card(
            "No browser build",
            "`cargo build` builds it, with no Node and no wasm. The crate embeds a prebuilt runtime of about 14 KB gzipped.",
        ))
        .child(card(
            "Typed from page to server",
            "Signal names, expressions, action paths, and their arguments come from Rust types, so a mismatch between the page and the server is a compile error.",
        ))
        .child(card(
            "Optimistic updates for free",
            "A statement changes the page in the browser, and an action call sent with it reaches the server after. Changing the page before the server answers takes no extra code.",
        ))
}

fn section_title(text: &str) -> Part {
    h2().id(prose::anchor(text))
        .css(
            Style::new()
                .font_size(rem(1.875))
                .line_height(LineHeight::Length(rem(2.25)))
                .font_weight(FontWeight::Number(700.0))
                .letter_spacing(rem(-0.025)),
        )
        .text(text)
}
