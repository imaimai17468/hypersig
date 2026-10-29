//! Actions with arguments, a busy flag, an error message, and an optimistic
//! update, for the page on actions.

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use axum::Router;
use hypersig::html::{attr, button, form, input, label, p, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
    saving: bool,
    error: String,
    saved: String,
    likes: i32,
}

hypersig::routes! {
    fn pages(Router<Likes>);
    const INDEX: Route = "/" => index;
    action SAVE: Action<Profile, String> = post "/save" => save;
    action LIKE: Action<Profile> = post "/like" => like;
}

#[derive(Clone, Default)]
struct Likes(Arc<AtomicI32>);

async fn index(likes: Likes, (): ()) -> Page<Profile> {
    let name = Profile::name();
    let saving = Profile::saving();
    let count = Profile::likes();
    let initial = Profile {
        name: String::new(),
        saving: false,
        error: String::new(),
        saved: String::new(),
        likes: likes.0.load(Ordering::Relaxed),
    };
    Page::new("Profile", initial)
        .child(
            form()
                .on_submit(
                    SAVE.call_with(name)
                        .indicator(saving)
                        .error(Profile::error()),
                )
                .child(label().text("Name ").child(input().bind(name)))
                .child(
                    button()
                        .attr(attr::TYPE, "submit")
                        .attr_when(attr::DISABLED, saving)
                        .text("Save"),
                )
                .child(p().bind_text(Profile::error()))
                .child(p().bind_text(Profile::saved())),
        )
        .child(
            button()
                // The count goes up at once, and the request follows.
                .on_click(count.set(count + 1).then(LIKE.call()))
                .text("Like ")
                .child(span().bind_text(count.to_text())),
        )
}

async fn save(_: Likes, name: String) -> Patches<Profile> {
    Profile::saved().patch(format!("Saved as {name}.")).into()
}

async fn like(likes: Likes, (): ()) -> Patches<Profile> {
    let total = likes.0.fetch_add(1, Ordering::Relaxed) + 1;
    Profile::likes().patch(total).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .with_state(Likes::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
