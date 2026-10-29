use hypersig::style::value::{deg, ms, secs, turn};

assert_eq!(deg(90).to_string(), "90deg");
assert_eq!(turn(0.25).to_string(), "0.25turn");
assert_eq!(ms(150).to_string(), "150ms");
assert_eq!(secs(1.5).to_string(), "1.5s");
// Unlike a length, a zero angle or duration keeps its unit.
assert_eq!(deg(0).to_string(), "0deg");
assert_eq!(ms(0).to_string(), "0ms");
