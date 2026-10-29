//! A search whose results the server renders as a fragment, for the page on
//! fragments.

use std::time::Duration;

use axum::Router;
use hypersig::html::{El, Event, Id, attr, input, li, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals, fragment};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

const FRUITS: &[&str] = &[
    "apple", "apricot", "banana", "cherry", "grape", "melon", "peach",
];

struct Results {
    query: String,
}

impl Fragment for Results {
    type Signals = Search;
    const ID: Id = Id::new("results");

    fn render(&self) -> El<Search> {
        ul().children(
            FRUITS
                .iter()
                .filter(|fruit| fruit.contains(self.query.as_str()))
                .map(|fruit| li().text(*fruit)),
        )
    }
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
    action SEARCH: Action<Search, String> = post "/search" => search;
}

async fn index((): (), (): ()) -> Page<Search> {
    let query = Search::query();
    Page::new(
        "Search",
        Search {
            query: String::new(),
        },
    )
    .child(
        input()
            .attr(attr::ARIA_LABEL, "Search fruit")
            .bind(query)
            .on_debounced(
                Event::Input,
                Duration::from_millis(200),
                SEARCH.call_with(query),
            ),
    )
    .child(fragment(&Results {
        query: String::new(),
    }))
}

async fn search((): (), query: String) -> Patches<Search> {
    Patches::new().element(&Results { query })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new()).hypersig_runtime();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
