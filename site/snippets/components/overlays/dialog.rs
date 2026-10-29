use hypersig::html::{Key, div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{DialogRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, DialogText, TextField, dialog_footer};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {
    name: String,
    status: String,
}

hypersig::dialog_ref!(const RENAME_DIALOG: DialogRef<Project>);
const RENAME: Dialog<Project> = Dialog::new(RENAME_DIALOG);
# fn main() {

let signals = Project { name: "hypersig".into(), status: String::new() };
let html = Page::new("Project", signals)
    // Ctrl+R, or Command+R on a Mac, opens the dialog without a button.
    .on_key(Key::Char('r').primary(), RENAME.open())
    .child(RENAME.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Rename"))
    .child(p().bind_text(Project::status()))
    .child(RENAME.dialog(
        "Rename project",
        "The new name shows everywhere the project appears.",
        div()
            .css(stack(rem(1)))
            .child(TextField::new("Name", Project::name()))
            .child(
                dialog_footer()
                    .child(RENAME.close(ButtonVariant::Outline, ButtonSize::Default).text("Cancel"))
                    .child(
                        RENAME
                            .close(ButtonVariant::Default, ButtonSize::Default)
                            .on_click(Project::status().set("Renamed the project."))
                            .text("Save"),
                    ),
            ),
        &DialogText::default(),
    ))
    .render();
# assert!(html.contains(r#"aria-haspopup="dialog""#));
# assert!(html.contains(r#"command="show-modal""#));
# assert!(html.contains(r#"command="close""#));
# assert!(html.contains(r#"closedby="any""#));
# assert!(html.contains(r#"aria-label="Close""#));
# }
