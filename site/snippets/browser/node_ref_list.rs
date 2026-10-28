use hypersig::html::{button, each_keyed, input, li, ul};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Card {
    id: i64,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    cards: Vec<Card>,
}

hypersig::node_ref!(const TITLE: NodeRef<Board>);
# fn main() {

// Every item carries `TITLE`, and each button selects the title in its own item.
let board = ul::<Board>().child(each_keyed(
    Board::cards(),
    |card| card.at(Card::id()),
    |card| {
        li().child(input().node_ref(TITLE).bind(card.at(Card::title())))
            .child(button().on_click(TITLE.select()).text("Rename"))
    },
));
# let _ = board.render();
# }
