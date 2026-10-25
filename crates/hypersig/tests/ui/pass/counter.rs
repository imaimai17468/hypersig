use hypersig::html::{button, div, input, span};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Other {
    flag: bool,
}

hypersig::routes! {
    fn actions(Router<()>);
    action RESET: Action<Counter> = post "/reset" => |(), ()| async { Patches::<Counter>::new() };
}

fn main() {
    let count = Counter::count();
    let _ = div::<Counter>()
        .show(count.gt(0).and(Counter::name().is_empty()))
        .child(span().bind_text(count.to_text()))
        .child(input().bind(Counter::name()))
        .child(button().on_click(count.set(count + 1).then(RESET.call())))
        .render();
    let _ = div::<Other>().show(Other::flag());
    let _ = actions;
}
