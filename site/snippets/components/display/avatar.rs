use hypersig::html::div;
use hypersig::{Page, SafeUrl, Signals};
use hypersig_ui::Avatar;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    picture: SafeUrl,
}

const ADA: SafeUrl = SafeUrl::new("https://example.com/ada.png");

let html = Page::new("Team", Team { picture: ADA })
    .child(
        div()
            // The picture comes from a signal, so the server can change it.
            .child(Avatar::new("Ada Lovelace", "AL").picture(Team::picture()))
            .child(Avatar::new("Grace Hopper", "GH")),
    )
    .render();
# assert!(html.contains(r#"role="img" aria-label="Ada Lovelace""#));
# assert!(html.contains(r#"src="https://example.com/ada.png""#));
# assert!(html.contains(r#"alt="AL""#));
# assert!(html.contains(">GH</span>"));
