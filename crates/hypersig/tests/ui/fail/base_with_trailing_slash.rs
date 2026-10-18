// A base path ends without `/`, so a route's path, which starts with one, follows it once.
use hypersig::Base;

const APP: Base = Base::new("/app/");

fn main() {
    let _ = APP;
}
