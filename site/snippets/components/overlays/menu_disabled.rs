use hypersig::{NodeRef, Signals};
use hypersig_ui::{Menu, MenuItem};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {
    owner: String,
}

hypersig::node_ref!(const PROJECT_ACTIONS: NodeRef<Project>);
const ACTIONS: Menu<Project> = Menu::new(PROJECT_ACTIONS);
# fn main() {

let menu = ACTIONS.menu([
    // `disabled` takes a condition, and `owner` is a `String`.
    MenuItem::new("Archive", Project::owner().set("")).disabled(Project::owner()),
]);
# }
