# use std::time::Duration;
# use hypersig::html::{Event, input};
# use serde::{Deserialize, Serialize};
use hypersig::{Action, Expr, Page, Patches, Route, Search, Signals};

// `/fruit` without a query string leaves `q` as `None`.
#[derive(Signals, Search, Serialize, Deserialize, Default)]
struct Filter {
    q: Option<String>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Fruit {
    query: String,
    matches: Vec<String>,
}

hypersig::routes! {
    fn pages(Router<()>);
    const FRUIT: Route<(), Filter> = "/fruit" => fruit;
}

const SEARCH: Action<Fruit, String> = Action::post("/search");

fn find(q: &str) -> Vec<String> {
    ["apple", "fig", "kiwi"].into_iter().filter(|name| name.contains(q)).map(String::from).collect()
}

// The page renders what the query string names, so Back and Forward load it again.
async fn fruit((): (), filter: Filter) -> Page<Fruit> {
    let q = filter.q.unwrap_or_default();
    let matches = find(&q);
    let query = Fruit::query();
    let shown = Expr::lit(&Filter::default()).with(Filter::q(), query.some());
    Page::new("Fruit", Fruit { query: q, matches }).child(
        input().bind(query).on_debounced(
            Event::Input,
            Duration::from_millis(250),
            // Each pause in typing replaces the URL rather than adding an entry.
            SEARCH.call_with(query).then(FRUIT.link_with(shown).replace_url()),
        ),
    )
}

const CLEAR: Action<Fruit> = Action::post("/clear");

// An action changes the URL too, here as a new entry after emptying the search.
async fn clear((): (), (): ()) -> Patches<Fruit> {
    let empty = Fruit::query().patch(String::new()).and(Fruit::matches().patch(find("")));
    Patches::from(empty).push_url(FRUIT.url(Filter::default()))
}
# let _ = (pages, CLEAR, clear);
