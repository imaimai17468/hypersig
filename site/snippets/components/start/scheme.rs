use hypersig::{Page, Signals, Stmt};
use hypersig_ui::{ButtonSize, ButtonVariant, select_field};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Scheme {
    System,
    Light,
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
    .css_match(
        scheme,
        [
            (Scheme::Light, hypersig_ui::light()),
            (Scheme::Dark, hypersig_ui::dark()),
        ],
    )
    .persist(scheme)
    .child(select_field(
        "Theme",
        scheme,
        [
            (Scheme::System, "System"),
            (Scheme::Light, "Light"),
            (Scheme::Dark, "Dark"),
        ],
    ))
    .child(
        hypersig_ui::button::<App>(ButtonVariant::Outline, ButtonSize::Default)
            // The change is animated as a view transition.
            .on_click(Stmt::view_transition(scheme.set(&Scheme::Dark)))
            .text("Dark"),
    )
    .render();
# assert!(html.contains("color-scheme:light"));
