use hypersig::style::Style;
use hypersig::style::value::raw;

// thread 'main' panicked: a raw CSS value holds no `;`, `{`, `}`, `<`,
// backslash, `/*`, or control character, and closes every quote and
// parenthesis it opens
let card = Style::new().margin(raw("0 auto; color: red"));
