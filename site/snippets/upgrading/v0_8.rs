# use hypersig::html::{Candidate, img, video};
# use hypersig::style::Style;
# use hypersig::style::value::pct;
# use hypersig::{Framing, Origin, Page, SafeUrl, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Widget {}
#
const PORTAL: Origin = Origin::new("https://portal.example");

let page = Page::new("Widget", Widget {})
//-    .framed_by(PORTAL)
    .framing(Framing::Origins(vec![PORTAL])) //+
    .child(
        img()
            .src(SafeUrl::new("/a-480.png"))
//-            .attr(attr::SRCSET, "/a-480.png 480w, /a-960.png 960w")
            .srcset([ //+
                (SafeUrl::new("/a-480.png"), Candidate::Width(480)), //+
                (SafeUrl::new("/a-960.png"), Candidate::Width(960)), //+
            ]) //+
//-            .attr(attr::STYLE, "max-width: 100%"),
            .css(Style::new().max_width(pct(100))), //+
    )
//-    .child(video().attr(attr::POSTER, "/intro.jpg"));
    .child(video().poster(SafeUrl::new("/intro.jpg"))); //+
# assert!(page.render().contains(r#"poster="/intro.jpg""#));
