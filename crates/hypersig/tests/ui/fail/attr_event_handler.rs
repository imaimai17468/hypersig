use hypersig::html::img;

fn main() {
    let _ = img::<()>().attr(hypersig::attr!("onError"), "alert(1)");
}
