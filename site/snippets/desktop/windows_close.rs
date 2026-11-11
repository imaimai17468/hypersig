# #[tokio::main]
# async fn main() -> Result<(), hypersig_desktop::Error> {
# use axum::Router;
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct NotePage { id: u64 }
# hypersig::routes! {
#     fn pages(Router<()>);
#     const NOTE: Route<u64> = "/notes/{id}" => note;
# }
# async fn note((): (), id: u64) -> Page<NotePage> { Page::new("Note", NotePage { id }) }
# let _ = pages(Router::new());
let fake = hypersig_desktop::testing::FakeOs::new();
let os = fake.os();
let first = os.windows().open(&NOTE.url(1)).await?;
let second = os.windows().open(&NOTE.url(2)).await?;
assert_ne!(first, second);

first.focus()?;
second.close()?;
assert_eq!(fake.open_windows(), [NOTE.url(1)]);
# Ok(())
# }
