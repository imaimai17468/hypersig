// A prefix ends without `/`, so a path joined after it cannot start with `//`.
use hypersig::Origin;

const BLOG: Origin = Origin::new("https://example.com").under("/blog/");

fn main() {
    let _ = BLOG;
}
