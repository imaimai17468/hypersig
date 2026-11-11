# use axum::Router;
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct ListPage {}
# #[derive(Signals, Serialize, Deserialize)]
# struct NotePage { id: u64 }
# hypersig::routes! {
#     fn pages(Router<()>);
#     const NOTE: Route<u64> = "/notes/{id}" => note;
# }
# async fn note((): (), id: u64) -> Page<NotePage> { Page::new("Note", NotePage { id }) }
# let _ = pages(Router::new());
use hypersig::html::{a, attr};

let link = a::<ListPage>()
    .href(NOTE.url(7))
    .attr(attr::TARGET, "_blank")
    .text("Open in a new window");
assert_eq!(
    link.render(),
    r#"<a href="/notes/7" target="_blank">Open in a new window</a>"#,
);
