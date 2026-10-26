//! Accessible components for hypersig, styled after shadcn/ui.
//!
//! Each component is a function or a small builder that returns an
//! [`El`](hypersig::html::El), styled with `hypersig::style` from the tokens
//! in [`token`]. A page adds [`global`] and [`theme`] once, and the
//! components follow the visitor's light or dark preference.
//!
//! The behavior a screen reader and a keyboard depend on comes from the
//! browser where the browser has it, and from the WAI-ARIA Authoring
//! Practices where it does not:
//!
//! | Component | Built on |
//! |---|---|
//! | [`button`], [`submit_button`], [`icon_button`] | `<button>`, with the icon button's name required |
//! | [`TextField`] | `<label>`, `<input>` or `<textarea>`, `aria-describedby`, `aria-invalid`, a live error |
//! | [`NumberField`] | the same, on `<input type="number">` with typed `min`, `max`, and `step` |
//! | [`checkbox`], [`switch`] | `<input type="checkbox">`, the switch with `role="switch"` |
//! | [`toggle`], [`icon_toggle`] | `<button>` with `aria-pressed` |
//! | [`ToggleGroup`] | the radio group pattern on toggle buttons, with roving focus |
//! | [`MultiToggleGroup`] | a `role="group"` of toggle buttons, with roving focus |
//! | [`radio_group`] | `<fieldset>`, `<legend>`, and radios that share a name |
//! | [`select_field`] | `<label>` and `<select>`, with a `base-select` picker where supported |
//! | [`TextField::in_form`], [`select_field_in_form`], [`radio_group_in_form`], [`checkbox_in_form`], [`switch_in_form`] | the control above bound to a field of a [`Form`](hypersig::Form), with its message tied by `aria-describedby` and `aria-invalid` |
//! | [`Slider`] | `<input type="range">` over a drawn track and thumb |
//! | [`Listbox`] | the listbox pattern, with roving focus |
//! | [`Combobox`] | the combobox pattern with list autocomplete and `aria-activedescendant` |
//! | [`Tabs`] | the tabs pattern, with roving focus |
//! | [`Chart`] | an `<svg>` hidden from assistive technology, beside a visually hidden `<table>` of the same rows |
//! | [`DonutChart`] | the same, with each row an arc of a ring |
//! | [`DataTable`] | `<table>` with `<th scope="col">` and `aria-sort`, and a tri-state checkbox for selecting rows |
//! | [`ServerTable`] | the same `<table>`, whose rows an action sends one page at a time |
//! | [`Accordion`] | `<details>` and `<summary>` |
//! | [`Dialog`] | `<dialog>` opened by invoker commands, centered or along an edge as a sheet |
//! | [`NavList`], [`Breadcrumb`], [`pagination`] | `<nav>` holding a list of links, with `aria-current` on the link to the page shown |
//! | [`Menu`] | the menu button pattern on a `popover` |
//! | [`Popover`] | a `popover` with `role="dialog"`, named by the button that opens it |
//! | [`Tooltip`] | the tooltip pattern on a `popover="hint"` |
//! | [`Toaster`] | a landmark region holding an `aria-live` list |
//! | [`progress`] | `role="progressbar"` |
//!
//! The ids that tie a label to its control, or a tab to its panel, are
//! derived from the signal the component is bound to. A dialog, a menu, a
//! popover, a tooltip, and a combobox take names in a `const` handle, as a
//! [`NodeRef`](hypersig::NodeRef) does, because the runtime or the browser
//! finds their elements by id.
//!
//! ```
//! use hypersig::{Page, Signals};
//! use hypersig_ui::{ButtonSize, ButtonVariant, TextField, button, card, card_content};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Signals, Serialize, Deserialize)]
//! struct Signup {
//!     name: String,
//! }
//!
//! let html = Page::new("Sign up", Signup { name: String::new() })
//!     .global(hypersig_ui::global())
//!     .theme(hypersig_ui::theme())
//!     .child(
//!         card().child(
//!             card_content()
//!                 .child(TextField::new("Name", Signup::name()))
//!                 .child(button(ButtonVariant::Default, ButtonSize::Default).text("Continue")),
//!         ),
//!     )
//!     .render();
//! assert!(html.contains(r#"for="ui-name-input""#));
//! ```
#![doc(
    html_logo_url = "https://imaimai17468.github.io/hypersig/icon.svg",
    html_favicon_url = "https://imaimai17468.github.io/hypersig/icon.svg"
)]

mod accordion;
mod alert;
mod badge;
mod button;
mod card;
mod chart;
mod choice;
mod combobox;
mod dialog;
mod donut;
mod field;
mod id;
mod listbox;
mod look;
mod menu;
mod navigation;
mod number;
mod popover;
mod progress;
mod select;
mod separator;
mod slider;
mod table;
mod tabs;
mod theme;
mod toast;
mod toggle;
pub mod token;
mod tooltip;

pub use accordion::Accordion;
pub use alert::{AlertVariant, alert, alert_description, alert_title};
pub use badge::{BadgeVariant, badge};
pub use button::{ButtonSize, ButtonVariant, button, button_style, icon_button, submit_button};
pub use card::{Level, card, card_content, card_description, card_footer, card_header, card_title};
pub use chart::Chart;
pub use choice::{
    checkbox, checkbox_in_form, radio_group, radio_group_in_form, switch, switch_in_form,
};
pub use combobox::{Combobox, ComboboxState};
pub use dialog::{Dialog, Side, dialog_footer};
pub use donut::DonutChart;
pub use field::{Autocomplete, InputKind, InputMode, TextField};
pub use listbox::Listbox;
pub use menu::{Menu, MenuItem};
pub use navigation::{Breadcrumb, NavList, PaginationText, pagination};
pub use number::{NumberField, Numeric};
pub use popover::Popover;
pub use progress::{progress, skeleton};
pub use select::{select_field, select_field_in_form};
pub use separator::separator;
pub use slider::Slider;
pub use table::{Column, DataTable, ServerTable, SortOrder, TableState, TableText};
pub use tabs::Tabs;
pub use theme::{dark, global, light, theme};
pub use toast::{Toast, ToastVariant, Toaster};
pub use toggle::{MultiToggleGroup, ToggleGroup, icon_toggle, toggle};
pub use tooltip::Tooltip;
