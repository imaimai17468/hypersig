use hypersig::style::value::quoted;
use hypersig::style::{PseudoElement, Style};

// error[E0277]: the trait bound `Condition: From<PseudoElement>` is not satisfied
let more = Style::new().on(PseudoElement::After, Style::new().content(quoted(" →")));
