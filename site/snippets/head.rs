# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use hypersig::{Asset, OpenGraph, Origin, Page, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    title: String,
}

# async fn post((): (), _: u32) -> Option<Page<Post>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const POST: Route<u32> = "/posts/{id}" => post;
}

// The scheme and host, checked when the constant is compiled.
const SITE: Origin = Origin::new("https://blog.example.com");

let page = Page::new("Hello", Post { title: "Hello".to_owned() })
    .description("The first post.")
    .canonical(SITE, &POST.url(1))
    .icon(Asset::new("/icon.svg"))
    .open_graph(OpenGraph::new().site_name("Blog").image(SITE, "/og/1.png", "A wave"));
let html = page.render();
assert!(html.contains(r#"<meta property="og:url" content="https://blog.example.com/posts/1">"#));
# let _ = pages;
