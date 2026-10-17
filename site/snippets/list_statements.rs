use hypersig::html::{button, div, p};
use hypersig::{Expr, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Task {
    title: String,
    done: bool,
    priority: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
}

let tasks = Board::tasks();
let new_task = Task { title: "Untitled".into(), done: false, priority: 0 };

let toolbar = div::<Board>()
    .child(button().on_click(tasks.push(Expr::lit(&new_task))).text("Add"))
    .child(button().on_click(tasks.set_all(Task::done(), true)).text("Finish all"))
    .child(button().on_click(tasks.retain(|task| !task.at(Task::done()))).text("Clear done"))
    .child(button().on_click(tasks.sort_by(|task| task.at(Task::priority()))).text("Sort"))
    .child(p().bind_text(tasks.count(|task| task.at(Task::done())).to_text()))
    .child(p().show(tasks.is_empty()).text("Nothing to do."));
# let _ = toolbar;
