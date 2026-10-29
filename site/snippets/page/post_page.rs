use axum::Router;
use hypersig::html::{article, h1, p};
use hypersig::{
    Asset, IntlLocale, OpenGraph, OpenGraphType, Origin, Page, Route, RouterExt, Signals,
};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    likes: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const POST: Route<u32> = "/posts/{id}" => post;
}

const SITE: Origin = Origin::new("https://blog.example.com");
const ICON: Asset = Asset::new("/icon.svg");

/// The page of post `id`, with what search results and link previews show.
fn post_page(id: u32, title: &str, summary: &str, body: &str) -> Page<Post> {
    Page::new(title, Post { likes: 0 })
        .lang(IntlLocale::En)
        .description(summary)
        .canonical(SITE, &POST.url(id))
        .icon(ICON)
        .open_graph(
            OpenGraph::new()
                .kind(OpenGraphType::Article)
                .site_name("Blog")
                .image(SITE, format!("/og/{id}.png"), title),
        )
        .child(article().child(h1().text(title)).child(p().text(body)))
}

async fn post((): (), id: u32) -> Option<Page<Post>> {
    // A post the store does not hold answers 404.
    (id == 1).then(|| post_page(id, "Hello", "The first post.", "Hello, world."))
}

let app: Router = pages(Router::new()).hypersig_runtime();
let html = post_page(1, "Hello", "The first post.", "Hello, world.").render();
assert!(html.contains(r#"<meta property="og:url" content="https://blog.example.com/posts/1">"#));
assert!(html.contains(r#"<meta property="og:type" content="article">"#));
# let _ = app;
