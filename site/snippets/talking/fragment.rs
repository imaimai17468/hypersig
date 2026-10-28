use hypersig::html::{El, Id, li, ul};
use hypersig::{Fragment, Patches, Signals, fragment};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

struct Cart {
    items: Vec<String>,
}

impl Fragment for Cart {
    type Signals = Shop;
    const ID: Id = Id::new("cart");

    fn render(&self) -> El<Shop> {
        ul().children(self.items.iter().map(|item| li().text(item.as_str())))
    }
}

let cart = Cart { items: vec!["tea".to_owned()] };

// In the page, the root carries the fragment's id.
assert_eq!(fragment(&cart).render(), r#"<ul id="cart"><li>tea</li></ul>"#);

// A handler sends the same fragment again, rendered from the new items.
let more = Cart { items: vec!["tea".to_owned(), "milk".to_owned()] };
let patches = Patches::new().element(&more);
assert!(patches.replaces::<Cart>());
