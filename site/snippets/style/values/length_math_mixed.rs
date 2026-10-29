use hypersig::style::value::{pct, rem};

// error[E0308]: mismatched types: expected `Percentage`, found `Length`
let inner = pct(100) - rem(2);
