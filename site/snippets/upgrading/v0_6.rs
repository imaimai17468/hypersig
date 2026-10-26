# use axum::Router;
# use hypersig::html::{button, div, input, span};
# use hypersig::{Action, NodeRef, Page, Patches, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Todos {
#     draft: String,
#     total: i32,
#     done: i32,
# }
#
//-const ADD: Action<Todos, String> = Action::post("/todos");
hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
    action ADD: Action<Todos, String> = post "/todos" => add; //+
}

//-const DRAFT: NodeRef<Todos> = NodeRef::new("draft");
hypersig::node_ref!(const DRAFT: NodeRef<Todos>); //+

async fn index((): (), (): ()) -> Page<Todos> {
    let left = Todos::total() - Todos::done();
    Page::new("Todos", Todos { draft: String::new(), total: 0, done: 0 }).child(
        div()
            .child(input().node_ref(DRAFT).bind(Todos::draft()))
            .child(button().on_click(ADD.call_with(Todos::draft())).text("Add"))
//-            .child(span().show(left.clone().gt(0)).bind_text(left.to_text())),
            .child(span().show(left.gt(0)).bind_text(left.to_text())), //+
    )
}

async fn add((): (), _draft: String) -> Patches<Todos> {
    Todos::draft().patch(String::new()).into()
}

# fn main() {
//-let app: Router = pages(Router::new()).action(ADD, add).hypersig_runtime();
let app: Router = pages(Router::new()).hypersig_runtime(); //+
# let _ = app;
# }
