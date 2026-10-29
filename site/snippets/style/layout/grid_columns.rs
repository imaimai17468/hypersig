use hypersig::style::pattern::grid;
use hypersig::style::value::rem;

// error[E0308]: mismatched types
// expected `u16`, found floating-point number
let cards = grid(2.5, rem(1));
