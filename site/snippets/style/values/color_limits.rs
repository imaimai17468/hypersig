use hypersig::style::value::Color;

// The top byte of an `#rrggbbaa` value is dropped, and the alpha goes with it.
assert_eq!(Color::hex(0x2563_eb80).to_string(), "#63eb80");
// A percentage above 100 counts as 100.
assert_eq!(
    Color::BLACK.opacity(150).to_string(),
    "color-mix(in srgb, black 100%, transparent)"
);
