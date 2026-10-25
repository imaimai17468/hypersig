use hypersig::DialogRef;

// A name written by hand could match another constant's, so `dialog_ref!` is
// the only way to make one.
const PALETTE: DialogRef<()> = DialogRef::new("palette");

fn main() {
    let _ = PALETTE;
}
