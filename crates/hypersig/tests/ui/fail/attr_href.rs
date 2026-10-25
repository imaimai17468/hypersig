use hypersig::html::a;

fn main() {
    let _ = a::<()>().attr(hypersig::attr!("HREF"), "javascript:alert(1)");
}
