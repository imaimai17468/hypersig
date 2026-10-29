use hypersig::{IntlLocale, Origin, Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct About {}

# async fn about((): (), (): ()) -> Option<Page<About>> { None }
# async fn kaisha((): (), (): ()) -> Option<Page<About>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const ABOUT: Route = "/about" => about;
    const KAISHA: Route = "/ja/kaisha" => kaisha;
}

const SITE: Origin = Origin::new("https://example.com");

let html = Page::new("About us", About {})
    .lang(IntlLocale::En)
    .alternate(SITE, &ABOUT.url(()), IntlLocale::En)
    .alternate(SITE, &KAISHA.url(()), IntlLocale::Ja)
    .alternate_default(SITE, &ABOUT.url(()))
    .render();
assert!(html.contains(concat!(
    r#"<link rel="alternate" hreflang="en" href="https://example.com/about">"#,
    r#"<link rel="alternate" hreflang="ja" href="https://example.com/ja/kaisha">"#,
    r#"<link rel="alternate" hreflang="x-default" href="https://example.com/about">"#,
)));
# let _ = pages;
