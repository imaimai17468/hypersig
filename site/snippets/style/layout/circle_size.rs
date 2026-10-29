use hypersig::style::pattern::circle;
use hypersig::style::value::pct;

// error[E0308]: mismatched types
// expected `Length`, found `Percentage`
let avatar = circle(pct(10));
