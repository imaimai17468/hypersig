use hypersig_desktop::App;

// error[E0080]: evaluation panicked: an application identifier is a reverse
// domain name such as `dev.example.notes`: two or more parts joined by dots,
// each of ASCII letters, digits, and hyphens
const APP: App = App::new("notes");
# let _ = APP;
