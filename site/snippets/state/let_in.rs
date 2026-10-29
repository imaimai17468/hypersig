use hypersig::html::p;
use hypersig::{Expr, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Scores {
    scores: Vec<i32>,
}

// A pass over the list, which `let_in` evaluates once.
let best = Scores::scores().max_of(|score| score.value()).unwrap_or(0);
let label: Expr<Scores, String> =
    best.let_in(|best| if_else(best.gt(90), "Top score", best.to_text().concat(" points")));

let headline = p::<Scores>().bind_text(label);
# let _ = headline;
