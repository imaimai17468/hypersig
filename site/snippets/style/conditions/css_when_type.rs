use hypersig::html::li;
use hypersig::style::Style;
use hypersig::style::value::Color;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    title: String,
}

// error[E0277]: `Sig<Todo, String>` is not an expression of type `bool` on the page whose signals are `Todo`
let item = li::<Todo>().css_when(Style::new().color(Color::GRAY), Todo::title());
