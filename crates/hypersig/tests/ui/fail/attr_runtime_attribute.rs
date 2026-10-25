use hypersig::html::div;

fn main() {
    let _ = div::<()>().attr(hypersig::attr!("data-hs-show"), "true");
}
