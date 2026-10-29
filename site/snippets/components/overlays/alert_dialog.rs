use hypersig::html::p;
use hypersig::{DialogRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, dialog_footer};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {
    deleted: bool,
}

hypersig::dialog_ref!(const DELETE_DIALOG: DialogRef<Project>);
const DELETE: Dialog<Project> = Dialog::new(DELETE_DIALOG);
# fn main() {

let html = Page::new("Project", Project { deleted: false })
    .child(DELETE.trigger(ButtonVariant::Destructive, ButtonSize::Default).text("Delete"))
    .child(p().show(Project::deleted()).text("Deleted the project."))
    .child(DELETE.alert_dialog(
        "Delete this project?",
        "The project and its deployments are removed. This cannot be undone.",
        dialog_footer()
            .child(DELETE.close(ButtonVariant::Outline, ButtonSize::Default).text("Cancel"))
            .child(
                DELETE
                    .close(ButtonVariant::Destructive, ButtonSize::Default)
                    .on_click(Project::deleted().set(true))
                    .text("Delete"),
            ),
    ))
    .render();
# assert!(html.contains(r#"role="alertdialog""#));
# assert!(!html.contains("closedby"));
# assert!(!html.contains(r#"aria-label="Close""#));
# }
