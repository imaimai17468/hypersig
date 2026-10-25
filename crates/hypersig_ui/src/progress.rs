//! Progress bars and loading placeholders.

use hypersig::html::{El, Role, attr, div};
use hypersig::style::value::{
    AnimationIterationCount, BorderRadius, Opacity, Overflow, ms, pct, px, rem,
};
use hypersig::style::{Keyframes, Media, Property, Style};
use hypersig::{Expr, IntoExpr};

use crate::look;
use crate::token;

/// A bar filled up to `percent`, a value from 0 to 100, named by `label`.
///
/// It is a `role="progressbar"` with `aria-valuenow` kept equal to the value,
/// so a screen reader reads the label and the percentage. A value below 0 or
/// above 100 is drawn and announced as 0 or 100.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::progress;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Upload {
///     percent: i32,
/// }
///
/// let bar = progress::<Upload>("Uploading report.pdf", Upload::percent());
/// assert!(bar.render().contains(r#"role="progressbar""#));
/// ```
#[must_use]
pub fn progress<S>(label: impl Into<String>, percent: impl IntoExpr<S, i32>) -> El<S> {
    let percent: Expr<S, i32> = percent.into_expr().max(0).min(100);
    div()
        .role(Role::ProgressBar)
        .attr(attr::ARIA_LABEL, label)
        .attr(attr::ARIA_VALUEMIN, "0")
        .attr(attr::ARIA_VALUEMAX, "100")
        .bind_attr(attr::ARIA_VALUENOW, percent.clone().to_text())
        .css(
            Style::new()
                .position(hypersig::style::value::Position::Relative)
                .height(rem(0.5))
                .width(pct(100))
                .overflow(Overflow::Hidden)
                .border_radius(BorderRadius::from(px(999)))
                .background_color(token::primary().var().opacity(20)),
        )
        .child(
            div()
                .css(
                    Style::new()
                        .height(pct(100))
                        .background_color(token::primary().var())
                        .merge(look::transition())
                        .transition_property(hypersig::style::value::raw("width")),
                )
                .bind_style(Property::Width, percent.to_text().concat("%")),
        )
}

/// A pulsing block in [`token::accent`] that stands in for content still
/// loading, hidden from assistive technology.
///
/// Size it with `.css`. Tell a screen reader that the content is loading
/// elsewhere, such as with `aria-busy` on the region it fills.
///
/// ```
/// use hypersig::Signals;
/// use hypersig::style::Style;
/// use hypersig::style::value::rem;
/// use hypersig_ui::skeleton;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct App {}
///
/// let line = skeleton::<App>().css(Style::new().height(rem(1)).width(rem(12)));
/// assert!(line.render().contains(r#"aria-hidden="true""#));
/// ```
#[must_use]
pub fn skeleton<S>() -> El<S> {
    let pulse = Keyframes::new("hs-ui-pulse").at(50, Style::new().opacity(Opacity::Number(0.5)));
    div().attr(attr::ARIA_HIDDEN, "true").css(
        Style::new()
            .border_radius(px(6))
            .background_color(token::accent().var())
            .keyframes(&pulse)
            .animation_duration(ms(2000))
            .animation_iteration_count(AnimationIterationCount::Infinite)
            .on(
                Media::ReducedMotion,
                Style::new().animation_iteration_count(AnimationIterationCount::Number(0.0)),
            ),
    )
}
