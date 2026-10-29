use hypersig::html::{attr, button, each_keyed, input, li, span, ul};
use hypersig::{Local, Sig, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Task {
    id: i32,
    title: String,
    done: bool,
    votes: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    tasks: Vec<Task>,
}

let list = ul::<Board>().child(each_keyed(
    Board::tasks(),
    |task| task.at(Task::id()),
    |task| {
        // A path from whichever item the browser is rendering.
        let votes: Sig<Board, i32, Local> = task.at(Task::votes());
        li().child(input().attr(attr::TYPE, "checkbox").bind(task.at(Task::done())))
            .child(input().bind(task.at(Task::title())))
            .child(span().bind_text(votes.to_text()))
            .child(button().on_click(votes.set(votes + 1)).text("+1"))
            .child(button().on_click(task.remove()).text("Remove"))
    },
));
# let _ = list;
