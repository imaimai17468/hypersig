use hypersig::style::value::px;

let gap: f32 = 0.1;
// The `f32` widens to the `f64` nearest to it, which is not 0.1.
assert_eq!(px(gap).to_string(), "0.10000000149011612px");
assert_eq!(px(0.1).to_string(), "0.1px");
