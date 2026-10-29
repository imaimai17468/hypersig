use hypersig::html::{button, each, form, input, li, span, ul};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    likes: i32,
    comments: Vec<String>,
    draft: String,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    action LIKE: Action<Post> = post "/like" => like;
    action COMMENT: Action<Post, String> = post "/comment" => comment;
}
# async fn like((): (), (): ()) -> Patches<Post> { Patches::new() }
# async fn comment((): (), _: String) -> Patches<Post> { Patches::new() }

let likes = Post::likes();
// The count goes up at once, and back down if the call fails.
let like = likes.set(likes + 1).then(
    LIKE.call()
        .error(Post::problem())
        .on_error(|_| likes.set(likes - 1)),
);

let draft = Post::draft();
// The call reads `draft` when it is sent, so emptying it afterwards does not
// change what the call sends.
let send = Post::comments()
    .push(draft)
    .then(COMMENT.call_with(draft).error(Post::problem()))
    .then(draft.set(""));

let page = ul::<Post>()
    .child(
        li().child(button().on_click(like).text("Like "))
            .child(span().bind_text(likes.to_text())),
    )
    .child(li().child(form().on_submit(send).child(input().bind(draft))))
    .child(each(Post::comments(), |text| li().bind_text(text)));
# let _ = (page, pages);
