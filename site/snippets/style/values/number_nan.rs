use hypersig::style::Style;

let done = 0.0_f64;
let total = 0.0_f64;
// Panics: 0.0 / 0.0 is NaN, which CSS cannot write.
let bar = Style::new().flex_grow(done / total);
