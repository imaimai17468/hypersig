use hypersig::html::{attr, div, p};
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::skeleton;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Report {
    loading: bool,
}

let loading = Report::loading();
let html = Page::new("Report", Report { loading: true })
    .child(
        div()
            // The region says it is loading, since the skeleton is hidden.
            .bind_attr(attr::ARIA_BUSY, loading.to_text())
            .child(
                div()
                    .show(loading)
                    .child(skeleton().css(Style::new().height(rem(1)).width(rem(16))))
                    .child(skeleton().css(Style::new().height(rem(1)).width(rem(12)))),
            )
            .child(p().show(!loading).text("The report is ready.")),
    )
    .render();
# assert!(html.contains(r#"aria-busy="true""#));
# assert!(html.contains(r#"aria-hidden="true""#));
