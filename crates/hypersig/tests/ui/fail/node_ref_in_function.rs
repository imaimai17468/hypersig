use hypersig::html::input;

// Two functions of one module could each declare `FIELD`, and the two
// constants would share the name `node_ref_in_function::FIELD`.
fn search() {
    hypersig::node_ref!(const FIELD: NodeRef<()>);
    let _ = input::<()>().node_ref(FIELD);
}

fn main() {
    search();
}
