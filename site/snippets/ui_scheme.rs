use hypersig::{Page, Signals, Stmt};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Scheme {
    System,
    Dark,
}

#[derive(Signals, Serialize, Deserialize)]
struct App {
    scheme: Scheme,
}

let scheme = App::scheme();
let html = Page::new("App", App { scheme: Scheme::System })
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    // The tokens go on `<body>`, and the choice outlives the page load.
    .css_match(scheme, [(Scheme::Dark, hypersig_ui::dark())])
    .persist(scheme)
    .child(
        button::<App>(ButtonVariant::Outline, ButtonSize::Default)
            .on_click(Stmt::view_transition(scheme.set(&Scheme::Dark)))
            .text("Dark"),
    )
    .render();
assert!(html.contains("data-hs-persist"));
