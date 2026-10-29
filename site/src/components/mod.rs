//! The Components section: setting up `hypersig_ui`, theming it, and one
//! page per component.

mod buttons;
mod data;
mod display;
mod forms;
mod layout;
mod navigation;
mod overlays;
mod pickers;
mod start;

use crate::Site;
use crate::doc::{Component, Doc};
use crate::prose::{Part, title};

/// The article of `page`.
pub fn content(site: Site, page: Component) -> Part {
    let head = title(Doc::Components(page).title(site));
    match page {
        // Get started
        Component::Installation => start::installation::page(site, head),
        Component::Theming => start::theming::page(site, head),
        Component::Tokens => start::tokens::page(site, head),
        Component::Handles => start::handles::page(site, head),
        Component::Translations => start::component_text::page(site, head),
        // Buttons
        Component::Button => buttons::button::page(site, head),
        Component::IconButton => buttons::icon_button::page(site, head),
        Component::LinkButton => buttons::link_button::page(site, head),
        Component::Toggle => buttons::toggle::page(site, head),
        Component::ToggleGroup => buttons::toggle_group::page(site, head),
        Component::MultiToggleGroup => buttons::multi_toggle_group::page(site, head),
        // Form fields
        Component::TextField => forms::text_field::page(site, head),
        Component::NumberField => forms::number_field::page(site, head),
        Component::DateField => forms::date_field::page(site, head),
        Component::Checkbox => forms::checkbox::page(site, head),
        Component::Switch => forms::switch::page(site, head),
        Component::RadioGroup => forms::radio_group::page(site, head),
        Component::Select => forms::select::page(site, head),
        Component::Slider => forms::slider::page(site, head),
        Component::FormFields => forms::form_fields::page(site, head),
        // Pickers
        Component::Listbox => pickers::listbox::page(site, head),
        Component::Combobox => pickers::combobox::page(site, head),
        // Layout
        Component::Tabs => layout::tabs::page(site, head),
        Component::Accordion => layout::accordion::page(site, head),
        Component::Collapsible => layout::collapsible::page(site, head),
        Component::Card => layout::card::page(site, head),
        Component::Separator => layout::separator::page(site, head),
        // Navigation
        Component::Navigation => navigation::nav_list::page(site, head),
        Component::Breadcrumb => navigation::breadcrumb::page(site, head),
        Component::Pagination => navigation::pagination::page(site, head),
        // Data
        Component::DataTable => data::data_table::page(site, head),
        Component::TableSorting => data::table_sorting::page(site, head),
        Component::TableFiltering => data::table_filtering::page(site, head),
        Component::TableSelection => data::table_selection::page(site, head),
        Component::ServerTable => data::server_table::page(site, head),
        Component::Chart => data::chart::page(site, head),
        Component::BarChart => data::bar_chart::page(site, head),
        Component::DonutChart => data::donut_chart::page(site, head),
        // Overlays
        Component::Dialog => overlays::dialog::page(site, head),
        Component::AlertDialog => overlays::alert_dialog::page(site, head),
        Component::Sheet => overlays::sheet::page(site, head),
        Component::Menu => overlays::menu::page(site, head),
        Component::Tooltip => overlays::tooltip::page(site, head),
        Component::Popover => overlays::popover::page(site, head),
        Component::Toast => overlays::toast::page(site, head),
        Component::ServerToasts => overlays::server_toasts::page(site, head),
        // Display
        Component::Alert => display::alert::page(site, head),
        Component::Badge => display::badge::page(site, head),
        Component::Progress => display::progress::page(site, head),
        Component::Skeleton => display::skeleton::page(site, head),
        Component::Spinner => display::spinner::page(site, head),
        Component::Avatar => display::avatar::page(site, head),
        Component::Kbd => display::kbd::page(site, head),
    }
}
