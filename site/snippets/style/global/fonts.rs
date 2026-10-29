use hypersig::html::{em, p};
use hypersig::style::value::{FontFamily, raw, url};
use hypersig::style::{Element, FontFace, Global, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

// One variable file for weights 100 to 900, and its italic face.
let inter = FontFace::new("Inter", url("/fonts/inter.woff2")).weights(100, 900);
let inter_italic = FontFace::new("Inter", url("/fonts/inter-italic.woff2"))
    .weights(100, 900)
    .italic();

let html = Page::new("Blog", Blog {})
    .global(
        Global::preflight()
            .rule(Element::Body, Style::new().font_family(inter.family()))
            // A list with fallbacks is written with `raw`, unchecked.
            .rule(Element::Code, Style::new().font_family(raw("\"JetBrains Mono\", monospace")))
            .rule(Element::Kbd, Style::new().font_family(FontFamily::UiMonospace))
            .font(inter)
            .font(inter_italic),
    )
    .child(p().child(em().text("Hello")))
    .render();
# assert!(html.contains("@font-face{font-family:\"Inter\";src:url(\"/fonts/inter.woff2\");font-display:swap;font-weight:100 900}"));
# assert!(html.contains("font-weight:100 900;font-style:italic}"));
# assert!(html.contains("body{font-family:\"Inter\";line-height:1.5}"));
