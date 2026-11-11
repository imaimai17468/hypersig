# use hypersig::SafeUrl;
# use hypersig_desktop::testing::FakeOs;
# let os = FakeOs::new().os();
// `file:` is not a scheme a `SafeUrl` takes, so this `const` fails to compile.
const HOSTS: SafeUrl = SafeUrl::new("file:///etc/hosts");
os.open_external(&HOSTS)?;
# Ok::<(), hypersig_desktop::Error>(())
