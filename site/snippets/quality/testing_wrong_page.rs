use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Votes {
    likes: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Poll {
    likes: i32,
}

let patches: Patches<Votes> = Votes::likes().patch(3).into();
// error[E0308]: mismatched types: expected `Sig<Votes, _>`, found `Sig<Poll, i32>`
let likes = patches.value_of(Poll::likes());
