use hypersig::html::{attr, button};

fn main() {
    let _ = button::<()>().attr(attr::ARIA_LABLE, "Close");
}
