use hypersig::html::{El, el};

/// Creates `<sl-rating>`, the element of `SlRating` in `components/rating/rating.js`.
#[must_use]
pub fn sl_rating<S>() -> El<S> {
    el("sl-rating")
}

/// The attributes and events of `<sl-rating>`.
pub mod sl_rating {
    /// The attributes of `<sl-rating>`.
    pub mod attr {
        /// `label`, of type `string`.
        pub const LABEL: &str = "label";
        /// `value`, of type `number`.
        pub const VALUE: &str = "value";
        /// `max`, of type `number`.
        pub const MAX: &str = "max";
        /// `precision`, of type `number`.
        pub const PRECISION: &str = "precision";
        /// `readonly`, of type `boolean`. Set it with `El::attr_when`.
        pub const READONLY: &str = "readonly";
        /// `disabled`, of type `boolean`. Set it with `El::attr_when`.
        pub const DISABLED: &str = "disabled";
        /// `getSymbol`, of type `(value: number) => string`.
        pub const GET_SYMBOL: &str = "getSymbol";
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
