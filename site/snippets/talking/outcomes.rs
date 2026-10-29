use hypersig::html::{button, each_keyed, li, span, ul};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Line {
    id: i32,
    name: String,
    problem: String,
    ordered: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    lines: Vec<Line>,
}

hypersig::routes! {
    fn pages(Router<()>);
    action ORDER: Action<Cart, i32> = post "/order" => order;
}
# async fn order((): (), _: i32) -> Patches<Cart> { Patches::new() }

// Each statement changes the line whose button sent the call.
let lines = ul::<Cart>().child(each_keyed(
    Cart::lines(),
    |line| line.at(Line::id()),
    |line| {
        let order = ORDER
            .call_with(line.at(Line::id()))
            .on_error(|message| line.at(Line::problem()).set(message))
            .on_success(line.at(Line::ordered()).set(true));
        li().child(span().bind_text(line.at(Line::name())))
            .child(button().on_click(order).text("Order"))
            .child(span().bind_text(line.at(Line::problem())))
    },
));
# let _ = (lines, pages);
