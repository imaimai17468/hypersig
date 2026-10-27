//! `#[derive(Segment)]` on a variant renamed to `..`, which the browser removes from a link.

use hypersig::Segment;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize)]
enum Dir {
    Here,
    #[serde(rename = "..")]
    Up,
}

fn main() {}
