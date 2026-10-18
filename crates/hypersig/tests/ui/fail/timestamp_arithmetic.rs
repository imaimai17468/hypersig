//! Arithmetic on a timestamp, which moves only by `add_millis`.
#![allow(unused)]
use hypersig::{Signals, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    posted: Timestamp,
}

fn main() {
    let _ = Post::posted().set(Post::posted() + 60_000);
}
