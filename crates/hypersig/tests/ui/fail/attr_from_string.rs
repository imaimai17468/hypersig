use hypersig::html::button;

fn main() {
    let _ = button::<()>().attr("aria-label", "Close");
}
