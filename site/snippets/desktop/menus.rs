# use hypersig_desktop::testing::FakeOs;
use hypersig_desktop::{Menu, MenuItem, Os};

/// Puts a File menu in the menu bar, and opens a note each time New is chosen.
fn file_menu(os: &Os) -> Result<(), hypersig_desktop::Error> {
    let new = MenuItem::new("New note");
    let mut clicks = new.clicks();
    os.menus([Menu::new("File").item(&new).separator().item(&MenuItem::new("Export"))])?;
    tokio::spawn(async move {
        while clicks.next().await {
            // Open a window for a new note here.
        }
    });
    Ok(())
}
# fn main() {
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# file_menu(&fake.os()).unwrap();
# assert_eq!(fake.menus()[0].labels(), ["New note", "Export"]);
# });
# }
