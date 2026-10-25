use hypersig::html::div;

fn main() {
    let _ = div::<()>().attr(hypersig::attr!("role"), "statsu");
}
