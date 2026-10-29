use hypersig::style::value::{quoted, url};

assert_eq!(url("/paper.png").to_string(), r#"url("/paper.png")"#);
assert_eq!(
    url("https://cdn.example/grain.webp").to_string(),
    r#"url("https://cdn.example/grain.webp")"#
);
assert_eq!(quoted("→").to_string(), r#""→""#);
// A quote, a backslash, `<`, and a control character are escaped.
assert_eq!(quoted("say \"hi\"").to_string(), r#""say \"hi\"""#);
assert_eq!(quoted("a\nb").to_string(), r#""a\a b""#);
assert_eq!(quoted("</style>").to_string(), r#""\3c /style>""#);
