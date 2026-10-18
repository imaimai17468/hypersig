// An origin ends at the host, so a canonical URL joined to it cannot hold the path twice.
use hypersig::Origin;

const SITE: Origin = Origin::new("https://example.com/blog/");

fn main() {
    let _ = SITE;
}
