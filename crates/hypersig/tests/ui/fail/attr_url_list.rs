use hypersig::html::img;

fn main() {
    let _ = img::<()>().attr(hypersig::attr!("srcset"), "javascript:alert(1) 1x");
    let _ = img::<()>().attr(hypersig::attr!("ping"), "https://tracker.example");
}
