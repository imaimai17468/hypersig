use hypersig::html::{Event, Orientation, Role, attr, button, div};
use hypersig::{Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    tab: i32,
}

let tab = Settings::tab();
let names = ["General", "Members", "Billing"];
let tabs = div::<Settings>()
    .role(Role::TabList)
    .roving_focus(Orientation::Horizontal)
    .children((0..3).map(|i| {
        button()
            .role(Role::Tab)
            // Tab enters the list at the selected tab.
            .bind_attr(attr::TABINDEX, if_else(tab.equals(i), "0", "-1"))
            .bind_attr(attr::ARIA_SELECTED, if_else(tab.equals(i), "true", "false"))
            // An arrow key moves focus, and focus selects the tab.
            .on(Event::Focus, tab.set(i))
            .text(names[i as usize])
    }));
# let _ = tabs.render();
