# use serde::{Deserialize, Serialize};
use hypersig::html::{a, h1, p};
use hypersig::{Page, PathSegment, Route, Signals, Unserved};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

hypersig::routes! {
    fn pages(Router<()>) else missing;
    const HOME: Route<PathSegment> = "/{lang}" => home;
    const ORDER: Route<(PathSegment, u32)> = "/{lang}/orders/{id}" in HOME => order;
}

# async fn home((): (), _: PathSegment) -> Page<Shop> { Page::new("Shop", Shop {}) }
// `None` sends the visitor the page of `missing`, with a 404.
async fn order((): (), (_lang, id): (PathSegment, u32)) -> Option<Page<Shop>> {
    (id < 100).then(|| Page::new(format!("Order {id}"), Shop {}))
}

// Every page request that reaches no page arrives here: an unknown path,
// a segment that does not parse, a `None`, and a bad query string.
async fn missing((): (), unserved: Unserved) -> Page<Shop> {
    let lang = unserved.within(HOME).unwrap_or(PathSegment::new("en"));
    let (title, back) = if lang.as_str() == "ja" {
        ("ページが見つかりません", "トップへ戻る")
    } else {
        ("Page not found", "Back to the shop")
    };
    Page::new(title, Shop {})
        .child(h1().text(title))
        // The `Display` names the path, for example
        // "no page is served at `/en/oders/3`".
        .child(p().text(unserved.to_string()))
        .child(p().child(a().href(HOME.link_with(lang)).text(back)))
}
# let _ = (pages, ORDER);
