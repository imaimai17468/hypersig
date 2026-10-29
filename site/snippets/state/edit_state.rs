# use serde::{Deserialize, Serialize};
use hypersig::Signals;
use hypersig::html::{button, each_keyed, input, li, span, ul};

#[derive(Signals, Serialize, Deserialize)]
struct Card {
    id: i64,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    cards: Vec<Card>,
    // At the root, where a patch of a card does not reach it.
    editing_id: Option<i64>,
}

let editing = Board::editing_id();
let board = ul::<Board>().child(each_keyed(
    Board::cards(),
    |card| card.at(Card::id()),
    |card| {
        let this = card.at(Card::id()).some();
        li().child(
            span()
                .show(editing.not_equals(this.clone()))
                .bind_text(card.at(Card::title())),
        )
        .child(input().show(editing.equals(this.clone())).bind(card.at(Card::title())))
        .child(button().on_click(editing.set(this)).text("Edit"))
    },
));
# let _ = board;
