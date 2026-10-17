# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Profile {}
use hypersig::html::{a, div, h1, p};

let card = div::<Profile>()
    .id("ada")
    .child(h1().text("Ada Lovelace"))
    .child(
        p().text("Wrote the first program. ")
            .child(a().attr("href", "/ada").text("Read more")),
    );

assert_eq!(
    card.render(),
    r#"<div id="ada"><h1>Ada Lovelace</h1><p>Wrote the first program. <a href="/ada">Read more</a></p></div>"#,
);
