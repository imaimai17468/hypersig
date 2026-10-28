use hypersig::SafeUrl;

// The scheme `javascript:` runs a script, so the constant fails to compile.
const HELLO: SafeUrl = SafeUrl::new("javascript:alert(1)");
# let _ = HELLO;
