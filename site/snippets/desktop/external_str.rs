# use hypersig_desktop::testing::FakeOs;
# let os = FakeOs::new().os();
// A `&str` is not a `&SafeUrl`: E0308.
os.open_external("https://example.com")?;
# Ok::<(), hypersig_desktop::Error>(())
