use hypersig::html::p;
use hypersig::style::value::{Color, FontFamily, LineHeight, url};
use hypersig::style::{Element, FontFace, Global, Media, PseudoClass, Selector, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {
    title: String,
}

let inter = FontFace::new("Inter", url("/fonts/inter.woff2")).weights(100, 900);
let family = inter.family();

let global = Global::preflight()
    .font(inter)
    .rule(Selector::Root, Style::new().on(Media::Dark, Style::new().color(Color::WHITE)))
    .rule(
        Element::Body,
        Style::new().font_family(family).line_height(LineHeight::Number(1.6)),
    )
    .rule(
        Element::A,
        Style::new()
            .color(Color::hex(0x0025_63eb))
            .on(PseudoClass::Hover, Style::new().color(Color::hex(0x001d_4ed8))),
    )
    .rule(Element::Code, Style::new().font_family(FontFamily::Monospace));

let html = Page::new("Blog", Blog { title: String::new() })
    .global(global)
    .child(p().text("Hello"))
    .render();
assert!(html.contains("@font-face"));
