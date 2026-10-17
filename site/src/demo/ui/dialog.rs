//! A dialog and an alert dialog, for the dialog page.

use hypersig::html::{div, p};
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, TextField, dialog_footer};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    name: String,
    status: String,
}

const RENAME: Dialog<Project> = Dialog::new("rename");
const DELETE: Dialog<Project> = Dialog::new("delete");

pub fn page() -> Page<Project> {
    let name = Project::name();
    let status = Project::status();
    let initial = Project {
        name: "hypersig".into(),
        status: String::new(),
    };
    Page::new("Project", initial).child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(
                        RENAME
                            .trigger(ButtonVariant::Outline, ButtonSize::Default)
                            .text("Rename"),
                    )
                    .child(
                        DELETE
                            .trigger(ButtonVariant::Destructive, ButtonSize::Default)
                            .text("Delete"),
                    ),
            )
            .child(p().attr("role", "status").bind_text(status))
            .child(RENAME.dialog(
                "Rename project",
                "The new name shows everywhere the project appears.",
                div()
                    .css(stack(rem(1)))
                    .child(TextField::new("Name", name))
                    .child(
                        dialog_footer()
                            .child(
                                RENAME
                                    .close(ButtonVariant::Outline, ButtonSize::Default)
                                    .text("Cancel"),
                            )
                            .child(
                                RENAME
                                    .close(ButtonVariant::Default, ButtonSize::Default)
                                    .on_click(status.set("Renamed the project."))
                                    .text("Save"),
                            ),
                    ),
            ))
            .child(DELETE.alert_dialog(
                "Delete this project?",
                "The project and its deployments are removed. This cannot be undone.",
                dialog_footer()
                    .child(
                        DELETE
                            .close(ButtonVariant::Outline, ButtonSize::Default)
                            .text("Cancel"),
                    )
                    .child(
                        DELETE
                            .close(ButtonVariant::Destructive, ButtonSize::Default)
                            .on_click(status.set("Deleted the project."))
                            .text("Delete"),
                    ),
            )),
    )
}
