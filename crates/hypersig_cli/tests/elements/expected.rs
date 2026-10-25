//! The custom elements of `custom-elements.json`, written by `cargo hypersig elements`.
//!
//! Each element has a function that creates it, a module `attr` with the
//! names of its attributes for `El::attr`, `El::bind_attr`, and
//! `El::attr_when`, and a module `event` with its events for
//! `El::on_custom`. The names and the `detail` types come from the manifest,
//! so they hold as far as the manifest describes the library's script. An
//! attribute or an event that a superclass or a mixin outside the manifest
//! declares is not here.
//!
//! Run the command again after updating the library, rather than editing
//! this file.

#![allow(dead_code, reason = "a page uses some of a library's elements")]

use hypersig::html::{El, el};

/// Creates `<md-elevated-button>`, the element of `MdElevatedButton` in `button/elevated-button.js`.
#[must_use]
pub fn md_elevated_button<S>() -> El<S> {
    el(hypersig::tag!("md-elevated-button"))
}

/// The attributes and events of `<md-elevated-button>`.
pub mod md_elevated_button {
    /// The attributes of `<md-elevated-button>`.
    pub mod attr {
        use hypersig::html::Attr;

        /// `soft-disabled`, of type `boolean`. Set it with `El::attr_when`.
        pub const SOFT_DISABLED: Attr = hypersig::attr!("soft-disabled");
        // `href` is left out: set it with `El::href`, which takes a `Link` or a `SafeUrl`.
        /// `download`, of type `string`.
        pub const DOWNLOAD: Attr = hypersig::attr!("download");
        /// `target`, of type `string`.
        pub const TARGET: Attr = hypersig::attr!("target");
        /// `trailing-icon`, of type `boolean`. Set it with `El::attr_when`.
        pub const TRAILING_ICON: Attr = hypersig::attr!("trailing-icon");
        /// `has-icon`, of type `boolean`. Set it with `El::attr_when`.
        pub const HAS_ICON: Attr = hypersig::attr!("has-icon");
    }
}

/// Creates `<sl-button>`, the element of `SlButton` in `components/button/button.js`.
#[must_use]
pub fn sl_button<S>() -> El<S> {
    el(hypersig::tag!("sl-button"))
}

/// The attributes and events of `<sl-button>`.
pub mod sl_button {
    /// The attributes of `<sl-button>`.
    pub mod attr {
        use hypersig::html::Attr;

        /// `variant`, of type `'default' | 'primary' | 'success' | 'neutral' | 'warning' | 'danger' | 'text'`.
        pub const VARIANT: Attr = hypersig::attr!("variant");
        /// `disabled`, of type `boolean`. Set it with `El::attr_when`.
        pub const DISABLED: Attr = hypersig::attr!("disabled");
        /// `type`, of type `'button' | 'submit' | 'reset'`.
        pub const TYPE: Attr = hypersig::attr!("type");
        // `href` is left out: set it with `El::href`, which takes a `Link` or a `SafeUrl`.
        /// `download`, of type `string | undefined`.
        pub const DOWNLOAD: Attr = hypersig::attr!("download");
        // `formaction` is left out: its value can run a script, so `hypersig::attr!` refuses it.
    }

    /// The events `<sl-button>` dispatches.
    pub mod event {
        use hypersig::html::CustomEvent;

        /// `sl-blur`. The manifest gives its `detail` no type, so the handler's `detail` is `()`.
        pub const SL_BLUR: CustomEvent<()> = CustomEvent::new("sl-blur");

        /// `sl-focus`. The manifest gives its `detail` no type, so the handler's `detail` is `()`.
        pub const SL_FOCUS: CustomEvent<()> = CustomEvent::new("sl-focus");

        /// `sl-invalid`. The manifest gives its `detail` no type, so the handler's `detail` is `()`.
        pub const SL_INVALID: CustomEvent<()> = CustomEvent::new("sl-invalid");
    }
}

/// Creates `<sl-rating>`, the element of `SlRating` in `components/rating/rating.js`.
#[must_use]
pub fn sl_rating<S>() -> El<S> {
    el(hypersig::tag!("sl-rating"))
}

/// The attributes and events of `<sl-rating>`.
pub mod sl_rating {
    /// The attributes of `<sl-rating>`.
    pub mod attr {
        use hypersig::html::Attr;

        /// `label`, of type `string`.
        pub const LABEL: Attr = hypersig::attr!("label");
        /// `value`, of type `number`.
        pub const VALUE: Attr = hypersig::attr!("value");
        /// `max`, of type `number`.
        pub const MAX: Attr = hypersig::attr!("max");
        /// `precision`, of type `number`.
        pub const PRECISION: Attr = hypersig::attr!("precision");
        /// `readonly`, of type `boolean`. Set it with `El::attr_when`.
        pub const READONLY: Attr = hypersig::attr!("readonly");
        /// `disabled`, of type `boolean`. Set it with `El::attr_when`.
        pub const DISABLED: Attr = hypersig::attr!("disabled");
        /// `getSymbol`, of type `(value: number) => string`.
        pub const GET_SYMBOL: Attr = hypersig::attr!("getSymbol");
    }

    /// The events `<sl-rating>` dispatches.
    pub mod event {
        use hypersig::Signals;
        use hypersig::html::CustomEvent;
        use serde::{Deserialize, Serialize};

        /// `sl-change`. The manifest gives its `detail` no type, so the handler's `detail` is `()`.
        pub const SL_CHANGE: CustomEvent<()> = CustomEvent::new("sl-change");

        /// `sl-hover`, whose `detail` is `{ phase: 'start' | 'move' | 'end', value: number }`.
        pub const SL_HOVER: CustomEvent<SlHoverDetail> = CustomEvent::new("sl-hover");

        /// The `detail` of `sl-hover`.
        #[derive(Debug, Signals, Serialize, Deserialize)]
        pub struct SlHoverDetail {
            /// `phase: 'start' | 'move' | 'end'`.
            pub phase: String,
            /// `value: number`.
            pub value: f64,
        }
    }
}

/// Creates `<test-picker>`, the element of `TestPicker` in `src/picker.js`.
#[must_use]
pub fn test_picker<S>() -> El<S> {
    el(hypersig::tag!("test-picker"))
}

/// The attributes and events of `<test-picker>`.
pub mod test_picker {
    /// The attributes of `<test-picker>`.
    pub mod attr {
        use hypersig::html::Attr;

        // `id` is left out: set it with `El::id`.
        // `onpick` is left out: its value can run a script, so `hypersig::attr!` refuses it.
        /// `max-items`, of type `number | undefined`.
        pub const MAX_ITEMS: Attr = hypersig::attr!("max-items");
        /// `max_items`, of type `number`.
        pub const MAX_ITEMS_2: Attr = hypersig::attr!("max_items");
    }

    /// The events `<test-picker>` dispatches.
    pub mod event {
        use hypersig::Signals;
        use hypersig::html::CustomEvent;
        use serde::{Deserialize, Serialize};

        /// `pick`, whose `detail` is `{ type: string; ids: number[]; note?: string; strict: true | false; self: string; range: { from: number } }`.
        pub const PICK: CustomEvent<PickDetail> = CustomEvent::new("pick");

        /// `count`, whose `detail` is `number | null`.
        pub const COUNT: CustomEvent<Option<f64>> = CustomEvent::new("count");

        /// `test-picker-selection-changed-after-the-user-confirms`, whose `detail` is `Array<string>`.
        pub const TEST_PICKER_SELECTION_CHANGED_AFTER_THE_USER_CONFIRMS: CustomEvent<Vec<String>> =
            CustomEvent::new("test-picker-selection-changed-after-the-user-confirms");

        /// The `detail` of `pick`.
        #[derive(Debug, Signals, Serialize, Deserialize)]
        pub struct PickDetail {
            /// `type: string`.
            pub r#type: String,
            /// `ids: number[]`.
            pub ids: Vec<f64>,
            /// `note?: string`.
            pub note: Option<String>,
            /// `strict: true | false`.
            pub strict: bool,
            // `self: string` is left out: its name is not a Rust field name.
            // `range: { from: number }` is left out: its type maps to no Rust type.
        }

        // "bad name" is left out: an event name holds no whitespace.
    }
}

/// Creates `<wa-stepper>`, the element of `WaStepper` in `components/stepper/stepper.js`.
#[must_use]
pub fn wa_stepper<S>() -> El<S> {
    el(hypersig::tag!("wa-stepper"))
}

/// The attributes and events of `<wa-stepper>`.
pub mod wa_stepper {
    /// The attributes of `<wa-stepper>`.
    pub mod attr {
        use hypersig::html::Attr;

        /// `orientation`, of type `'horizontal' | 'vertical' | 'auto'`.
        pub const ORIENTATION: Attr = hypersig::attr!("orientation");
        /// `linear`, of type `boolean`. Set it with `El::attr_when`.
        pub const LINEAR: Attr = hypersig::attr!("linear");
        /// `did-ssr`.
        pub const DID_SSR: Attr = hypersig::attr!("did-ssr");
    }

    /// The events `<wa-stepper>` dispatches.
    #[allow(
        non_snake_case,
        reason = "a field of a `detail` is named as the script names it"
    )]
    pub mod event {
        use hypersig::Signals;
        use hypersig::html::CustomEvent;
        use serde::{Deserialize, Serialize};

        /// `detail`. The manifest's type for it, `WaStepChangeEvent`, maps to no Rust type, so the handler's `detail` is `()`.
        pub const DETAIL: CustomEvent<()> = CustomEvent::new("detail");

        /// `wa-before-step-change`, whose `detail` is `{ name: string, previousName: string | null, step: WaStep, previousStep: WaStep | null }`.
        pub const WA_BEFORE_STEP_CHANGE: CustomEvent<WaBeforeStepChangeDetail> =
            CustomEvent::new("wa-before-step-change");

        /// `wa-step-change`, whose `detail` is `{ name: string, previousName: string | null, step: WaStep, previousStep: WaStep | null }`.
        pub const WA_STEP_CHANGE: CustomEvent<WaStepChangeDetail> =
            CustomEvent::new("wa-step-change");

        /// The `detail` of `wa-before-step-change`.
        #[derive(Debug, Signals, Serialize, Deserialize)]
        pub struct WaBeforeStepChangeDetail {
            /// `name: string`.
            pub name: String,
            /// `previousName: string | null`.
            pub previousName: Option<String>,
            // `step: WaStep` is left out: its type maps to no Rust type.
            // `previousStep: WaStep | null` is left out: its type maps to no Rust type.
        }

        /// The `detail` of `wa-step-change`.
        #[derive(Debug, Signals, Serialize, Deserialize)]
        pub struct WaStepChangeDetail {
            /// `name: string`.
            pub name: String,
            /// `previousName: string | null`.
            pub previousName: Option<String>,
            // `step: WaStep` is left out: its type maps to no Rust type.
            // `previousStep: WaStep | null` is left out: its type maps to no Rust type.
        }
    }
}
