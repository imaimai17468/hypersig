use hypersig::html::select;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Sort {
    Newest,
    Cheapest,
}

#[derive(Signals, Serialize, Deserialize)]
struct Catalog {
    sort: Sort,
}

let picker = select::<Catalog>().bind_select(
    Catalog::sort(),
    [(Sort::Newest, "Newest first"), (Sort::Cheapest, "Lowest price")],
);
# let html = Page::new("Catalog", Catalog { sort: Sort::Newest }).child(picker).render();
# assert!(html.contains(">Newest first</option>") && html.contains("selected"), "{html}");
