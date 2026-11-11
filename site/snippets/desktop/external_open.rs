# use hypersig::html::{a, button, div, p};
# use hypersig::{Action, Page, Patches, SafeUrl, Signals};
# use hypersig_desktop::Os;
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
#[derive(Signals, Serialize, Deserialize)]
struct About {
    status: String,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn actions(Router<AppState>);
    action REPORT: Action<About> = post "/report" => report;
}

const SOURCE: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");
const NEW_ISSUE: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig/issues/new");

fn about() -> Page<About> {
    Page::new("About", About { status: String::new() }).child(
        div()
            // Clicked, this opens in the person's browser and the window stays.
            .child(a().href(SOURCE).text("Source"))
            .child(button().on_click(REPORT.call()).text("Report a problem"))
            .child(p().bind_text(About::status())),
    )
}

/// Opens the form for a new issue in the person's browser.
async fn report(state: AppState, (): ()) -> Patches<About> {
    match state.os.open_external(&NEW_ISSUE) {
        Ok(()) => Patches::new(),
        Err(error) => About::status().patch(format!("Not opened: {error}")).into(),
    }
}
# fn main() {
# let _ = actions;
# let _ = about().render();
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# let _ = report(AppState { os: fake.os() }, ()).await;
# assert_eq!(fake.opened_externally(), [NEW_ISSUE]);
# });
# }
