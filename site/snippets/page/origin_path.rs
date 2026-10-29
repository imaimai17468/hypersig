use hypersig::Origin;

// error[E0080]: evaluation panicked: an origin ends at the host and port
const SITE: Origin = Origin::new("https://example.com/blog/");
# let _ = SITE;
