use hypersig::html::el;

fn main() {
    let _ = el::<()>(hypersig::tag!("SCRIPT")).text("alert(1)");
}
