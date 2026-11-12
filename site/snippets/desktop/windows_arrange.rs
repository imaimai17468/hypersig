# #[tokio::main]
# async fn main() -> Result<(), hypersig_desktop::Error> {
# use axum::Router;
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct CallPage {}
# hypersig::routes! {
#     fn pages(Router<()>);
#     const CALL: Route = "/call" => call;
# }
# async fn call((): (), (): ()) -> Page<CallPage> { Page::new("Call", CallPage {}) }
# let _ = pages(Router::new());
use hypersig_desktop::{Layer, Placement};

let fake = hypersig_desktop::testing::FakeOs::new();
let os = fake.os();
os.windows().open(&CALL.url(())).await?;

// A button in the call's page shrinks its window and keeps it on top.
if let Some(window) = os.windows().focused().await? {
    window.place(Placement::Normal)?;
    window.resize(320, 180)?;
    window.layer(Layer::AlwaysOnTop)?;

    let arranged = fake.window(&window).expect("the window is open");
    assert_eq!(arranged.size, Some((320, 180)));
    assert_eq!(arranged.layer, Layer::AlwaysOnTop);
}
# Ok(())
# }
