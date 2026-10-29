use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    likes: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action LIKE: Action<Post> = post "/like" => like;
}
# async fn like((): (), (): ()) -> Patches<Post> { Patches::new() }

let likes = Post::likes();
// error[E0277]: `Expr<Post, String>` is not an expression of type `i32` on the page whose signals are `Post`
let like = likes
    .set(likes + 1)
    .then(LIKE.call().on_error(|message| likes.set(message)));
# let _ = (button::<Post>().on_click(like), pages);
