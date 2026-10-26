//! A page with an action and the tests that call them, for the page on
//! testing. `site/Cargo.toml` sets `test = true` for this example, so
//! `cargo test` runs the tests below.

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use axum::Router;
use axum::http::StatusCode;
use hypersig::html::{button, p, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Votes {
    likes: i32,
    error: String,
}

hypersig::routes! {
    fn pages(Router<Likes>);
    const INDEX: Route = "/" => index;
    action LIKE: Action<Votes, i32> = post "/like" => like;
}

#[derive(Clone, Default)]
struct Likes(Arc<AtomicI32>);

async fn index(likes: Likes, (): ()) -> Page<Votes> {
    let initial = Votes {
        likes: likes.0.load(Ordering::Relaxed),
        error: String::new(),
    };
    Page::new("Votes", initial)
        .child(
            button()
                .on_click(LIKE.call_with(1).error(Votes::error()))
                .text("Like ")
                .child(span().bind_text(Votes::likes().to_text())),
        )
        .child(p().bind_text(Votes::error()))
}

async fn like(likes: Likes, by: i32) -> Result<Patches<Votes>, (StatusCode, &'static str)> {
    if !(1..=5).contains(&by) {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Like 1 to 5 times."));
    }
    let total = likes.0.fetch_add(by, Ordering::Relaxed) + by;
    Ok(Votes::likes().patch(total).into())
}

fn app(likes: Likes) -> Router {
    pages(Router::new()).hypersig_runtime().with_state(likes)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app(Likes::default())).await
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt as _;
    use tower::ServiceExt as _;

    use super::*;

    #[tokio::test]
    async fn the_page_starts_at_the_stored_count() {
        let likes = Likes::default();
        likes.0.store(7, Ordering::Relaxed);
        let page = index(likes, ()).await;
        assert_eq!(page.value_of(Votes::likes()), 7);
        assert!(page.render().contains(">7</span>"));
    }

    #[tokio::test]
    async fn a_like_adds_to_the_count() {
        let patches = like(Likes::default(), 2).await.unwrap();
        assert_eq!(patches.value_of(Votes::likes()), Some(2));
    }

    #[tokio::test]
    async fn more_than_five_likes_are_refused() {
        let refused = like(Likes::default(), 6).await;
        assert_eq!(
            refused.err(),
            Some((StatusCode::UNPROCESSABLE_ENTITY, "Like 1 to 5 times."))
        );
    }

    #[tokio::test]
    async fn the_router_answers_the_action_with_its_patches() {
        let response = app(Likes::default())
            .oneshot(LIKE.request(&3))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let expected = Patches::from(Votes::likes().patch(3)).to_event_stream();
        assert_eq!(body, expected.as_bytes());
    }

    #[tokio::test]
    async fn the_router_serves_the_page() {
        let request = Request::get(INDEX.url(()).as_str())
            .body(Body::empty())
            .unwrap();
        let response = app(Likes::default()).oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
