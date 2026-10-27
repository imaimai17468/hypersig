//! A `PathSegment` written as `..`, which the browser removes from a link.

use hypersig::{PathSegment, Rest};

const UP: PathSegment = PathSegment::new("..");
const ESCAPE: Rest = Rest::new("docs/../../etc");

fn main() {
    let _ = (UP, ESCAPE);
}
