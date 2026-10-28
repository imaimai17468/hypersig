use hypersig::html::{each_keyed, li, ul};
use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Card {
    id: i32,
    tags: Vec<String>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    cards: Vec<Card>,
}

let board = ul::<Board>().child(each_keyed(
    Board::cards(),
    |card| card.at(Card::id()),
    |card| {
        // error[E0599]: no method named `patch_append` found for struct `Sig<Board, Vec<String>, hypersig::Local>` in the current scope
        let tag: Patches<Board> = card.at(Card::tags()).patch_append(&["urgent".to_owned()]).into();
        li()
    },
));
