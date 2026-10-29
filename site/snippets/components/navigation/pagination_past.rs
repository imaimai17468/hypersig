use hypersig::{Route, Signals};
use hypersig_ui::{PaginationText, pagination};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Catalog {}

# async fn products((): (), _: u32) -> Option<hypersig::Page<Catalog>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const PRODUCTS: Route<u32> = "/products/{page}" => products;
}

// Panics: the current page is between 1 and the number of pages, not 6 of 5.
let pager = pagination::<Catalog>(6, 5, |n| PRODUCTS.link_with(&n), &PaginationText::default());
# let _ = pages;
