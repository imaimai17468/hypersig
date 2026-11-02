//! A unit struct for a page with no signals, which serializes to `null` rather than an object.

use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home;

fn main() {}
