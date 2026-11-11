# fn main() -> Result<(), hypersig_desktop::Error> {
use hypersig_desktop::App;

const APP: App = App::new("dev.example.notes");

// `~/Library/Application Support/dev.example.notes` on macOS.
let dir = APP.data_dir()?;
assert!(dir.ends_with("dev.example.notes"));
# Ok(())
# }
