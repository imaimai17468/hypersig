use hypersig::html::p;
use hypersig::{IntlLocale, Page, PluralCategory, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    count: i32,
}

let count = Cart::count();
// `form` runs once per category, and the browser keeps the form of the current count.
let label = count.to_text().concat(count.plural(IntlLocale::En, |category| match category {
    PluralCategory::One => " item",
    _ => " items",
}));

let html = Page::new("Cart", Cart { count: 1 })
    .lang(IntlLocale::En)
    .child(p().bind_text(label))
    .render();
# assert!(html.contains(">1 item</p>"), "{html}");
# // A page in `ja` with a `NumberFormat` and no plural or date passes the check.
# let _ = Page::new("カート", Cart { count: 1 })
#     .lang(IntlLocale::Ja)
#     .child(p().bind_text(Cart::count().format(&hypersig::NumberFormat::new())))
#     .render();
