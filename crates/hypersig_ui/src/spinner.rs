//! Spinners: a turning ring that marks work in progress.

use hypersig::html::{El, Role, attr, span};
use hypersig::style::value::{
    AlignItems, AnimationIterationCount, AnimationTimingFunction, Display, ms, raw, rem,
};
use hypersig::style::{Keyframes, Media, Style};

use crate::look::{self, icon};

/// A turning ring named `label`, which marks work that has no known length.
///
/// It is a `role="status"` with `aria-label` set to `label`, and the ring
/// inside it is hidden from assistive technology. A screen reader reads
/// `label` where it reaches the spinner, and whether it also reads it as the
/// spinner appears differs between screen readers, so say in text when the
/// work finishes, such as in a [`Toast`](crate::Toast). The ring turns in
/// the current text color, and four times slower for a visitor who asks for
/// reduced motion. Use [`progress`](crate::progress) where the share done is
/// known.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::spinner;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Upload {
///     busy: bool,
/// }
///
/// let busy = spinner::<Upload>("Uploading").show(Upload::busy());
/// assert!(busy.render().contains(r#"role="status""#));
/// ```
#[must_use]
pub fn spinner<S>(label: impl Into<String>) -> El<S> {
    let spin = Keyframes::new("hs-ui-spin").at(100, Style::new().rotate(raw("360deg")));
    span()
        .role(Role::Status)
        .attr(attr::ARIA_LABEL, label)
        .css(
            Style::new()
                .display(Display::InlineFlex)
                .align_items(AlignItems::Center),
        )
        .child(
            icon(look::icon::LOADER_CIRCLE, rem(1)).css(
                Style::new()
                    .keyframes(&spin)
                    .animation_duration(ms(1000))
                    .animation_timing_function(AnimationTimingFunction::Linear)
                    .animation_iteration_count(AnimationIterationCount::Infinite)
                    .on(
                        Media::ReducedMotion,
                        Style::new().animation_duration(ms(4000)),
                    ),
            ),
        )
}
