# use hypersig::SafeUrl;
# use hypersig_desktop::ErrorKind;
# use hypersig_desktop::testing::FakeOs;
# let os = FakeOs::new().os();
// Both are `SafeUrl`s, and neither names a scheme.
for url in [SafeUrl::new("/notes"), SafeUrl::new("//example.com")] {
    let error = os.open_external(&url).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::NotExternal);
}
