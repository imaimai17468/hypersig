# use axum::http::StatusCode;
# use hypersig::html::{button, each, li, span, ul};
# use hypersig::{Action, Patches, Signals};
# use serde::{Deserialize, Serialize};
#
#[derive(Signals, Serialize, Deserialize)]
struct Line {
    id: i32,
    dish: String,
    problem: String, //+
}

# #[derive(Signals, Serialize, Deserialize)]
# struct Order {
#     lines: Vec<Line>,
# }
#
# #[derive(Clone)]
# struct AppState;
#
# impl AppState {
#     fn sold_out(&self, _line: i32) -> bool {
#         true
#     }
# }
#
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     action SEND: Action<Order, i32> = post "/order/send" => send;
# }
#
# let _ = ul().child(
each(Order::lines(), |line| {
    li().child(span().bind_text(line.at(Line::dish())))
        .child(button().text("Send to the kitchen").on_click(
//-            SEND.call_with(line.at(Line::id())),
            SEND.call_with(line.at(Line::id())) //+
                // The failure lands on the line whose button was pressed. //+
                .on_error(|message| line.at(Line::problem()).set(message)) //+
                .on_success(line.at(Line::problem()).set("")), //+
        ))
        .child(span().bind_text(line.at(Line::problem()))) //+
})
# );

async fn send(state: AppState, line: i32) -> Result<Patches<Order>, (StatusCode, &'static str)> {
    if state.sold_out(line) { //+
        return Err((StatusCode::CONFLICT, "Sold out. Ask the guest for another dish.")); //+
    } //+
    Ok(Patches::new())
}
# let _ = pages;
