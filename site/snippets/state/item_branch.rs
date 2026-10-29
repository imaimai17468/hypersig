use hypersig::html::{each, li, span, ul};
use hypersig::{Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    title: String,
    done: bool,
    urgent: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
}

// The closure runs once, so a Rust `if` here would choose for every item
// at once. Expressions choose per item in the browser.
let list = ul::<Board>().child(each(Board::tasks(), |task| {
    let done = task.at(Task::done());
    li().child(span().bind_text(task.at(Task::title())))
        .child(span().bind_text(if_else(done, " (done)", "")))
        .child(span().show(task.at(Task::urgent()).and(!done)).text(" Urgent"))
}));
# let _ = list;
