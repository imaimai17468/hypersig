use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Signals, Serialize, Deserialize)]
struct Post {}

let post = Page::new("Hello", Post {})
    .json_ld(&json!({ "@type": "Article", "headline": "Hello" }));
assert!(post.render().contains(
    r#"<script type="application/ld+json">{"@type":"Article","headline":"Hello"}</script>"#
));

// A page that search results should leave out, such as a cart.
let cart = Page::new("Cart", Post {}).noindex().nofollow();
assert!(cart.render().contains(r#"<meta name="robots" content="noindex, nofollow">"#));
