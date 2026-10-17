//! A menu button, for the menu page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Menu, MenuItem};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    archived: bool,
    status: String,
}

const ACTIONS: Menu<Project> = Menu::new("actions");

pub fn page() -> Page<Project> {
    let archived = Project::archived();
    let status = Project::status();
    let initial = Project {
        archived: false,
        status: String::new(),
    };
    Page::new("Project", initial).child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .child(
                        ACTIONS
                            .trigger(ButtonVariant::Outline, ButtonSize::Default)
                            .text("Project actions"),
                    )
                    .child(ACTIONS.menu([
                        MenuItem::new("Rename", status.set("Renamed the project.")),
                        MenuItem::new("Duplicate", status.set("Duplicated the project.")),
                        MenuItem::new(
                            "Archive",
                            archived.set(true).then(status.set("Archived the project.")),
                        )
                        .disabled(archived),
                        MenuItem::new("Delete", status.set("Deleted the project.")).destructive(),
                    ])),
            )
            .child(p().attr("role", "status").bind_text(status)),
    )
}
