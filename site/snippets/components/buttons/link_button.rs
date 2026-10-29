use hypersig::html::{a, div, h1};
use hypersig::{Page, Route, SafeUrl, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button_style};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {}

# async fn home((): (), (): ()) -> Option<Page<Home>> { None }
# async fn signup((): (), (): ()) -> Option<Page<Home>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => home;
    const SIGNUP: Route = "/signup" => signup;
}

// In a `const`, a URL whose scheme can run a script fails to compile.
const SOURCE: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");

let page = Page::new("Home", Home {}).child(
    div()
        .child(h1().text("Plan your week"))
        // A link to a page of this site, from its route.
        .child(
            a().href(SIGNUP.link())
                .css(button_style(ButtonVariant::Default, ButtonSize::Lg))
                .text("Sign up"),
        )
        // A link to another site.
        .child(
            a().href(SOURCE)
                .css(button_style(ButtonVariant::Outline, ButtonSize::Lg))
                .text("Source"),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"href="/signup""#));
# let _ = (pages, HOME);
