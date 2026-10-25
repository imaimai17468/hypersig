use hypersig::html::label;

fn main() {
    let _ = label::<()>().attr(hypersig::attr!("for"), "emial");
}
