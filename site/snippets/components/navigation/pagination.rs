use hypersig::html::{li, ul};
use hypersig::{Page, Route, Signals};
use hypersig_ui::{PaginationText, pagination};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Catalog {}

hypersig::routes! {
    fn pages(Router<()>);
    const PRODUCTS: Route<u32> = "/products/{page}" => products;
}

const PER_PAGE: usize = 20;

async fn products((): (), page: u32) -> Option<Page<Catalog>> {
    let all: Vec<String> = (1..=95).map(|n| format!("Product {n}")).collect();
    let count = u32::try_from(all.len().div_ceil(PER_PAGE)).ok()?;
    // A page outside 1 to `count` answers 404 here, where `pagination` would panic.
    if page == 0 || page > count.max(1) {
        return None;
    }
    let start = (page as usize - 1) * PER_PAGE;
    let shown = all.iter().skip(start).take(PER_PAGE).map(|name| li().text(name));
    let pager = pagination(page, count, |n| PRODUCTS.link_with(&n), &PaginationText::default());
    Some(Page::new("Products", Catalog {}).child(ul().children(shown)).child(pager))
}

# let _ = pages;
# let html = futures::executor::block_on(products((), 3)).unwrap().render();
# assert!(html.contains(r#"href="/products/3" aria-current="page""#));
# assert!(html.contains(r#"href="/products/2" rel="prev""#));
# assert!(html.contains(r#"href="/products/5""#));
# assert!(futures::executor::block_on(products((), 6)).is_none());
