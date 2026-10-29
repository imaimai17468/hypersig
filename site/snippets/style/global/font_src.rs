use hypersig::style::FontFace;

// error[E0308]: mismatched types: expected `Url`, found `&str`
let inter = FontFace::new("Inter", "/fonts/inter.woff2");
