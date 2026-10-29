# use hypersig::Page;
use hypersig::{Route, Search, SearchScalar, Segment, Signals};
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Color {
    Red,
    Blue,
}

// A page number of 1 or more. Reading the query string checks it.
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(try_from = "i32", into = "i32")]
struct PageNumber(i32);

impl TryFrom<i32> for PageNumber {
    type Error = &'static str;

    fn try_from(number: i32) -> Result<Self, Self::Error> {
        if number >= 1 { Ok(Self(number)) } else { Err("a page number starts at 1") }
    }
}

impl From<PageNumber> for i32 {
    fn from(number: PageNumber) -> Self {
        number.0
    }
}

// serde writes `PageNumber` as an `i32`, so one value of the query string holds it.
impl SearchScalar for PageNumber {}

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    q: String,
    page: Option<PageNumber>,
    color: Vec<Color>,
}

# #[derive(Signals, Serialize, Deserialize)]
# struct Shop { page: i32 }
hypersig::routes! {
    fn pages(Router<()>);
    const SHOP: Route<(), Filter> = "/shop" => shop;
}

// `/shop?q=tea` reaches the handler with `page: None`, and `/shop?page=0` gets a 400.
async fn shop((): (), filter: Filter) -> Page<Shop> {
    let page = filter.page.map_or(1, i32::from);
    Page::new("Shop", Shop { page })
}

let filter = Filter {
    q: "green tea".to_owned(),
    page: None,
    color: vec![Color::Red, Color::Blue],
};
assert_eq!(SHOP.url(filter).as_str(), "/shop?color=red&color=blue&q=green%20tea");
# let _ = pages;
