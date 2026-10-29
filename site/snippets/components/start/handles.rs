use hypersig::html::{div, p};
use hypersig::{DialogRef, NodeRef, Page, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Combobox, ComboboxState, ComboboxText, Dialog, DialogText, Menu,
    MenuItem, Popover, Tooltip, dialog_footer,
};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    archived: bool,
    country: ComboboxState,
}

// Each handle is a `const` built from a reference that names its element.
hypersig::dialog_ref!(const EDIT_PROFILE: DialogRef<Profile>);
const EDIT: Dialog<Profile> = Dialog::new(EDIT_PROFILE);

hypersig::node_ref!(const PROFILE_ACTIONS: NodeRef<Profile>);
const ACTIONS: Menu<Profile> = Menu::new(PROFILE_ACTIONS);

hypersig::node_ref!(const PROFILE_DETAILS: NodeRef<Profile>);
const DETAILS: Popover<Profile> = Popover::new(PROFILE_DETAILS);

hypersig::node_ref!(const ARCHIVE_TIP: NodeRef<Profile>);
const ARCHIVE: Tooltip<Profile> = Tooltip::new(ARCHIVE_TIP);

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Profile>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Profile>);
const COUNTRY: Combobox<Profile> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);
# fn main() {

// The button and the element it opens are built from the same handle.
let page = Page::new("Profile", Profile { archived: false, country: ComboboxState::new("") })
    .child(
        div()
            .child(EDIT.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Edit"))
            .child(EDIT.dialog(
                "Edit profile",
                "Changes are saved when you press Save.",
                dialog_footer()
                    .child(EDIT.close(ButtonVariant::Default, ButtonSize::Default).text("Save")),
                &DialogText::default(),
            ))
            .child(ACTIONS.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Actions"))
            .child(ACTIONS.menu([MenuItem::new("Archive", Profile::archived().set(true))]))
            .child(DETAILS.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Details"))
            .child(DETAILS.popover().child(p().text("Joined in 2024.")))
            .child(ARCHIVE.icon_trigger(ButtonVariant::Ghost, "Archive").text("A"))
            .child(ARCHIVE.tooltip("Moves the profile out of the list."))
            .child(COUNTRY.render(
                "Country",
                Profile::country(),
                ["France", "Japan", "Kenya"],
                &ComboboxText::default(),
            )),
    );
# let html = page.render();
# assert!(html.contains(r#"aria-haspopup="menu""#));
# }
