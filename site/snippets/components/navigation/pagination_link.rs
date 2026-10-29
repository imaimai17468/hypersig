use hypersig::Signals;
use hypersig_ui::{PaginationText, pagination};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Catalog {}

// error[E0308]: mismatched types: expected `Link<Catalog>`, found `String`
let pager = pagination::<Catalog>(
    3,
    5,
    |page| format!("/products/{page}"),
    &PaginationText::default(),
);
