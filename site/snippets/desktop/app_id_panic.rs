use hypersig_desktop::App;

// Panics: an application identifier is a reverse domain name such as
// `dev.example.notes`: two or more parts joined by dots, each of ASCII
// letters, digits, and hyphens
let app = App::new("dev.example.my_notes");
# let _ = app;
