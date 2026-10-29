use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::ViewTransitionName:
//     From<&str>` is not satisfied
let header = Style::new().view_transition_name("site-header");
