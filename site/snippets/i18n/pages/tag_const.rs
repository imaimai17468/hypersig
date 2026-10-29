use hypersig::LanguageTag;

// error[E0080]: evaluation panicked: a subtag holds only ASCII letters and
// digits, separated by `-`
const EN_US: LanguageTag = LanguageTag::new("en_US");
# let _ = EN_US;
