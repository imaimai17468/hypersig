//! `#[derive(Segment)]` on two variants that serde writes as one segment, which a URL could not tell apart.

use hypersig::Segment;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    #[serde(rename = "en")]
    English,
}

fn main() {}
