# use serde::{Deserialize, Serialize};
use hypersig::html::{div, each_keyed, li, ul};
use hypersig::{Action, Patches, RouterExt, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    posts: Vec<Post>,
    // The page the next load asks for: 1 at first, because page 0 is in the HTML.
    page: i32,
    loading: bool,
    more: bool,
}

const MORE: Action<Feed, i32> = Action::get("/feed/more");
const PER_PAGE: usize = 20;

# fn load_page(_page: i32) -> Vec<Post> { vec![Post { id: 21, title: "Hello".to_owned() }] }
let feed = div::<Feed>()
    .child(ul().child(each_keyed(
        Feed::posts(),
        |post| post.at(Post::id()),
        |post| li().bind_text(post.at(Post::title())),
    )))
    // Comes into view below the last post, and stays hidden while a load runs.
    .child(
        div()
            .show(Feed::more().and(!Feed::loading()))
            .on_visible(MORE.call_with(Feed::page()).indicator(Feed::loading())),
    );

async fn more((): (), page: i32) -> Patches<Feed> {
    let posts = load_page(page);
    Patches::new().list(Feed::posts().patch_append(&posts)).signals(
        Feed::page()
            .patch(page + 1)
            .and(Feed::more().patch(posts.len() == PER_PAGE)),
    )
}

let app: axum::Router = axum::Router::new().action(MORE, more);
# let _ = (feed, app);
