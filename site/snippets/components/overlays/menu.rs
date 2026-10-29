use hypersig::html::{Role, div, p};
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Menu, MenuItem};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {
    archived: bool,
    status: String,
}

hypersig::node_ref!(const PROJECT_ACTIONS: NodeRef<Project>);
const ACTIONS: Menu<Project> = Menu::new(PROJECT_ACTIONS);
# fn main() {

let archived = Project::archived();
let status = Project::status();
let html = Page::new("Project", Project { archived: true, status: String::new() })
    .child(
        div()
            .child(ACTIONS.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Actions"))
            .child(ACTIONS.menu([
                MenuItem::new("Duplicate", status.set("Duplicated the project.")),
                MenuItem::new("Archive", archived.set(true).then(status.set("Archived.")))
                    .disabled(archived),
                MenuItem::new("Delete", status.set("Deleted the project.")).destructive(),
            ])),
    )
    .child(p().role(Role::Status).bind_text(status))
    .render();
# assert!(html.contains(r#"aria-haspopup="menu""#));
# assert!(html.contains(r#"popover="auto" role="menu""#));
# assert!(html.contains(r#"role="menuitem" tabindex="-1""#));
# assert!(html.contains(r#"disabled="""#));
# }
