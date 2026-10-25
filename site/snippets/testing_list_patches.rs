use hypersig::{Item, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Debug, PartialEq)]
struct Card {
    id: i32,
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    cards: Vec<Card>,
}

let by_id = |card: Item<Board, Card>| card.at(Card::id());

// What a handler that renames card 3 and deletes card 7 returns.
let renamed = Card { id: 3, title: "Ship it".to_owned() };
let patches: Patches<Board> = Board::cards()
    .patch_upsert(&[renamed], by_id)
    .and(Board::cards().patch_remove(&[7], by_id))
    .into();

assert_eq!(
    patches.items_of(Board::cards()),
    [Card { id: 3, title: "Ship it".to_owned() }],
);
assert_eq!(patches.removed_keys_of(Board::cards(), by_id), [7]);
