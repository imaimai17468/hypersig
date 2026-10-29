use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::TransitionDuration:
//     From<{integer}>` is not satisfied
let fade = Style::new().transition_duration(150);
