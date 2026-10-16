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
//! | [`checkbox`], [`switch`] | `<input type="checkbox">`, the switch with `role="switch"` |
//! | [`radio_group`] | `<fieldset>`, `<legend>`, and radios that share a name |
//! | [`select_field`] | `<label>` and `<select>` |
//! | [`Tabs`] | the tabs pattern, with roving focus |
//! | [`Accordion`] | `<details>` and `<summary>` |
//! | [`Dialog`] | `<dialog>` opened by invoker commands |
//! | [`Menu`] | the menu button pattern on a `popover` |
//! | [`progress`] | `role="progressbar"` |
//!
//! The ids that tie a label to its control, or a tab to its panel, are
//! derived from the signal the component is bound to. A dialog and a menu
//! have no signal, so they take a name in a `const` handle, as a
//! [`NodeRef`](hypersig::NodeRef) does.
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

mod accordion;
mod alert;
mod badge;
mod button;
mod card;
mod choice;
mod dialog;
mod field;
mod id;
mod look;
mod menu;
mod progress;
mod select;
mod separator;
mod tabs;
mod theme;
pub mod token;

pub use accordion::Accordion;
pub use alert::{AlertVariant, alert, alert_description, alert_title};
pub use badge::{BadgeVariant, badge};
pub use button::{ButtonSize, ButtonVariant, button, button_style, icon_button, submit_button};
pub use card::{Level, card, card_content, card_description, card_footer, card_header, card_title};
pub use choice::{checkbox, radio_group, switch};
pub use dialog::{Dialog, dialog_footer};
pub use field::{Autocomplete, InputKind, TextField};
pub use menu::{Menu, MenuItem};
pub use progress::{progress, skeleton};
pub use select::select_field;
pub use separator::separator;
pub use tabs::Tabs;
pub use theme::{dark, global, light, theme};
