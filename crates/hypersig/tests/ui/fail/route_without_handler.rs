//! A route in `routes!` that names no handler.

hypersig::routes! {
    fn pages(Router<()>);
    const ORPHAN: Route = "/orphan";
}

fn main() {}
