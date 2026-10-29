use hypersig::style::value::rem;
use hypersig::style::{Global, Style};

// error[E0277]: the trait bound `Selector: From<&str>` is not satisfied
let global = Global::new().rule(".card", Style::new().padding(rem(1)));
