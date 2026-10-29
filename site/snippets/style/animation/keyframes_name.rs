use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::AnimationName:
//     From<&str>` is not satisfied
let spinner = Style::new().animation_name("spin");
