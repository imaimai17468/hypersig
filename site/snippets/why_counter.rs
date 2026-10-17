# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
# async fn history((): (), _: i32) -> Option<Page<Counter>> { None }
use hypersig::html::{a, button, div, span};
use hypersig::{Action, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const HISTORY: Route<i32> = "/history/{count}" => history;
}

const RESET: Action<Counter> = Action::post("/reset");

let count = Counter::count();
let page = Page::new("Counter", Counter { count: 0 }).child(
    div()
        .child(button().on_click(count.set(count + 1)).text("+1"))
        .child(span().bind_text(count.to_text()))
        .child(button().on_click(RESET.call()).text("Reset"))
        .child(a().href(HISTORY.link_with(count)).text("History")),
);
# let html = page.render();
# let body = &html[html.find("<body").unwrap()..];
# assert_eq!(body, concat!(
#     r#"<body data-hs-signals="{&quot;count&quot;:0}"><div>"#,
#     r#"<button data-hs-on-click="[&quot;set&quot;,&quot;count&quot;,[&quot;add&quot;,[&quot;sig&quot;,&quot;count&quot;],[&quot;lit&quot;,1]]]">+1</button>"#,
#     r#"<span data-hs-text="[&quot;str&quot;,[&quot;sig&quot;,&quot;count&quot;]]">0</span>"#,
#     r#"<button data-hs-on-click="[&quot;call&quot;,&quot;POST&quot;,&quot;/reset&quot;,null,null,null]">Reset</button>"#,
#     r#"<a href="/history/0" data-hs-attr="[[&quot;href&quot;,[&quot;concat&quot;,[&quot;lit&quot;,&quot;/history/&quot;],[&quot;uri&quot;,[&quot;sig&quot;,&quot;count&quot;]]]]]">History</a>"#,
#     "</div></body></html>",
# ));
# let _ = pages;
