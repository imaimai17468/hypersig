use hypersig::html::{button, each_keyed, li, span, ul, when};
use hypersig::testing::nth;
use hypersig::{NodeRef, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct List {
    todos: Vec<Todo>,
    editing: bool,
}

hypersig::node_ref!(const DONE: NodeRef<List>);
hypersig::node_ref!(const MARK: NodeRef<List>);
hypersig::node_ref!(const HINT: NodeRef<List>);

# fn main() {
let todos = vec![
    Todo { id: 1, title: "Tea".to_owned(), done: false },
    Todo { id: 2, title: "Milk".to_owned(), done: false },
];
let page = Page::new("Todos", List { todos, editing: false })
    .child(ul().child(each_keyed(List::todos(), |todo| todo.at(Todo::id()), |todo| {
        li().child(
            button()
                .node_ref(DONE)
                .on_click(todo.at(Todo::done()).set(true))
                .bind_text(todo.at(Todo::title())),
        )
        .child(span().node_ref(MARK).show(todo.at(Todo::done())).text("done"))
    })))
    .child(when(List::editing(), span().node_ref(HINT).text("Press Enter to save")));

let mut browser = page.simulate();
// The template puts `DONE` on every item, and a `when` that is false shows nothing.
assert_eq!(browser.count(DONE), 2);
assert_eq!(browser.count(HINT), 0);

browser.click(nth(DONE, 1));
assert_eq!(browser.element(nth(DONE, 1)).text(), "Milk");
assert!(!browser.element(nth(MARK, 0)).is_visible());
assert!(browser.element(nth(MARK, 1)).is_visible());
# }
