//! `#[derive(Segment)]` on an enum with a variant that holds a value.

use hypersig::Segment;

#[derive(Segment)]
enum Tab {
    Overview,
    Member(i32),
}

fn main() {}
