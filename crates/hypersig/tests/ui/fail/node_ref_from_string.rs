use hypersig::NodeRef;

// A name written by hand could match another constant's, so `node_ref!` is
// the only way to make one.
const FIELD: NodeRef<()> = NodeRef::new("field");

fn main() {
    let _ = FIELD;
}
