// `og:locale` takes a language and a territory joined by `_`, so a BCP 47 tag does not fit it.
use hypersig::OpenGraphLocale;

const EN: OpenGraphLocale = OpenGraphLocale::new("en-US");

fn main() {
    let _ = EN;
}
