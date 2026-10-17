# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Profile {}
use hypersig::Route;
use hypersig::html::{a, div, h1, p};

# async fn ada((): (), (): ()) -> Option<hypersig::Page<Profile>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const ADA: Route = "/ada" => ada;
}

let card = div::<Profile>()
    .id("ada")
    .child(h1().text("Ada Lovelace"))
    .child(
        p().text("Wrote the first program. ")
            .child(a().href(ADA.link()).text("Read more")),
    );

assert_eq!(
    card.render(),
    r#"<div id="ada"><h1>Ada Lovelace</h1><p>Wrote the first program. <a href="/ada">Read more</a></p></div>"#,
);
# let _ = pages;
