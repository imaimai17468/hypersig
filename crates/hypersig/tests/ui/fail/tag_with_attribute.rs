use hypersig::html::el;

fn main() {
    let _ = el::<()>(hypersig::tag!("img onerror=alert(1)"));
}
