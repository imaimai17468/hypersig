//! New versions of the application, found, downloaded, and installed with
//! `cargo-packager-updater`, under the `updater` feature.
//!
//! The endpoint answers in the format `cargo-packager` documents: `204 No
//! Content` when there is no newer version, or JSON naming the version, its
//! notes, and, for the system and architecture the endpoint's `{{target}}`
//! and `{{arch}}` stand for, the URL of the package and its minisign
//! signature. A package whose signature the public key does not verify is
//! refused before anything is installed.
//!
//! On Linux, `cargo-packager-updater` sets `SSL_CERT_FILE` and
//! `SSL_CERT_DIR` with `std::env::set_var` when they are unset, the first
//! time it checks, while other threads of the application may read the
//! environment. Setting both before [`App::run`](crate::App::run) avoids it.

use cargo_packager_updater::semver::Version;
use cargo_packager_updater::url::Url;
use cargo_packager_updater::{Config, UpdaterBuilder};

use crate::error::Error;

/// Where an application finds its new versions, and the key that signs them.
#[derive(Debug, Clone, Copy)]
pub struct Updates {
    version: &'static str,
    endpoint: &'static str,
    public_key: &'static str,
}

impl Updates {
    /// The versions after `version` that `endpoint` announces, signed by `public_key`.
    ///
    /// `version` is the running one, `env!("CARGO_PKG_VERSION")` in the
    /// application's crate. `endpoint` may hold `{{target}}`, `{{arch}}`,
    /// and `{{current_version}}`. `public_key` is the one `cargo packager
    /// signer generate` writes.
    ///
    /// # Panics
    ///
    /// Panics when `version` is not a semantic version, when `endpoint` is
    /// not `https://` followed by a host, or `http://` to `127.0.0.1` or
    /// `localhost` for a test on this machine, and when `public_key` is not
    /// a minisign public key in base64. In a `const`, each is a compile
    /// error.
    ///
    /// ```compile_fail,E0080
    /// # use hypersig_desktop::Updates;
    /// const UPDATES: Updates = Updates::new("1.0", "https://updates.example.com", "");
    /// ```
    #[must_use]
    pub const fn new(
        version: &'static str,
        endpoint: &'static str,
        public_key: &'static str,
    ) -> Self {
        assert!(
            is_version(version.as_bytes()),
            "`version` is a semantic version such as `1.2.3`"
        );
        assert!(
            is_endpoint(endpoint.as_bytes()),
            "`endpoint` is `https://` and a host, or `http://` to `127.0.0.1` or `localhost`"
        );
        assert!(
            is_public_key(public_key.as_bytes()),
            "`public_key` is a minisign public key, in base64, as `cargo packager signer generate` writes it"
        );
        Self {
            version,
            endpoint,
            public_key,
        }
    }

    /// The newer version the endpoint announces, or `None`.
    ///
    /// # Errors
    ///
    /// Fails when the endpoint does not parse as a URL, and when it cannot
    /// be reached or answers with something else.
    pub(crate) async fn check(self) -> Result<Option<Update>, Error> {
        let version = Version::parse(self.version).map_err(Error::update)?;
        let endpoint = Url::parse(self.endpoint).map_err(Error::update)?;
        let config = Config {
            endpoints: vec![endpoint],
            pubkey: self.public_key.to_owned(),
            windows: None,
        };
        tokio::task::spawn_blocking(move || {
            let updater = UpdaterBuilder::new(version, config).build()?;
            updater.check()
        })
        .await
        .map_err(Error::update)?
        .map(|update| update.map(|inner| Update { inner }))
        .map_err(Error::update)
    }
}

/// A newer version, from [`Os::updates`](crate::Os::updates).
pub struct Update {
    inner: cargo_packager_updater::Update,
}

impl std::fmt::Debug for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Update")
            .field("version", &self.inner.version)
            .finish_non_exhaustive()
    }
}

impl Update {
    /// The version the endpoint announced, written as a semantic version
    /// such as `1.1.0`.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.inner.version
    }

    /// The release notes, when the endpoint sent some.
    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.inner.body.as_deref()
    }

    /// Downloads the package and checks its signature.
    ///
    /// # Errors
    ///
    /// Fails when the package cannot be downloaded, and when the public key
    /// does not verify its signature.
    pub async fn download(self) -> Result<Downloaded, Error> {
        tokio::task::spawn_blocking(move || {
            let bytes = self.inner.download()?;
            Ok::<_, cargo_packager_updater::Error>(Downloaded {
                update: self.inner,
                bytes,
            })
        })
        .await
        .map_err(Error::update)?
        .map_err(Error::update)
    }
}

/// A package [`Update::download`] downloaded and verified.
pub struct Downloaded {
    update: cargo_packager_updater::Update,
    bytes: Vec<u8>,
}

impl std::fmt::Debug for Downloaded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Downloaded")
            .field("version", &self.update.version)
            .field("bytes", &self.bytes.len())
            .finish_non_exhaustive()
    }
}

impl Downloaded {
    /// Installs the package over the running application.
    ///
    /// The new version runs once the application starts again.
    ///
    /// On Windows it does not return: cargo-packager-updater 0.2.3 starts
    /// the installer and ends the process with `std::process::exit(0)`.
    /// That skips the shutdown of [`App::run`](crate::App::run), so
    /// requests in progress get no time to finish and the window's
    /// `localStorage` is not waited for. On macOS the `.app` bundle and on
    /// Linux the `AppImage` are replaced, and the application keeps running.
    ///
    /// # Errors
    ///
    /// Fails when the package cannot be installed.
    pub async fn install(self) -> Result<(), Error> {
        tokio::task::spawn_blocking(move || self.update.install(self.bytes))
            .await
            .map_err(Error::update)?
            .map_err(Error::update)
    }
}

/// Whether `text` is `MAJOR.MINOR.PATCH`, each a number without a leading
/// zero, then optionally `-` and a pre-release and `+` and build metadata,
/// each dot-separated identifiers of ASCII letters, digits, and `-`.
const fn is_version(text: &[u8]) -> bool {
    let mut at = 0;
    let mut part = 0;
    while part < 3 {
        let start = at;
        while at < text.len() && text[at].is_ascii_digit() {
            at += 1;
        }
        if at == start || (text[start] == b'0' && at - start > 1) {
            return false;
        }
        part += 1;
        if part < 3 {
            if at == text.len() || text[at] != b'.' {
                return false;
            }
            at += 1;
        }
    }
    if at < text.len() && text[at] == b'-' {
        at = identifiers(text, at + 1, true);
        if at == 0 {
            return false;
        }
    }
    if at < text.len() && text[at] == b'+' {
        at = identifiers(text, at + 1, false);
        if at == 0 {
            return false;
        }
    }
    at == text.len()
}

/// Where the dot-separated identifiers that start at `at` end, or 0 when
/// one is empty or holds a byte other than an ASCII letter, digit, or `-`.
/// In a pre-release, a numeric identifier has no leading zero.
const fn identifiers(text: &[u8], mut at: usize, pre_release: bool) -> usize {
    loop {
        let start = at;
        let mut numeric = true;
        while at < text.len() && (text[at].is_ascii_alphanumeric() || text[at] == b'-') {
            numeric = numeric && text[at].is_ascii_digit();
            at += 1;
        }
        if at == start || (pre_release && numeric && text[start] == b'0' && at - start > 1) {
            return 0;
        }
        if at == text.len() || text[at] != b'.' {
            return at;
        }
        at += 1;
    }
}

/// Whether `text` is `https://` and a host, or `http://` to `127.0.0.1` or
/// `localhost`, each followed by the end, `:`, or `/`.
const fn is_endpoint(text: &[u8]) -> bool {
    if let Some(host) = after(text, b"https://") {
        return !host.is_empty() && host[0] != b'/' && host[0] != b':';
    }
    let Some(host) = after(text, b"http://") else {
        return false;
    };
    let rest = match after(host, b"127.0.0.1") {
        Some(rest) => rest,
        None => match after(host, b"localhost") {
            Some(rest) => rest,
            None => return false,
        },
    };
    rest.is_empty() || rest[0] == b':' || rest[0] == b'/'
}

/// `text` after `prefix`, when it starts with it.
const fn after<'a>(text: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    if text.len() < prefix.len() {
        return None;
    }
    let mut at = 0;
    while at < prefix.len() {
        if text[at] != prefix[at] {
            return None;
        }
        at += 1;
    }
    Some(text.split_at(prefix.len()).1)
}

/// Whether `text` is standard base64 of a minisign public key file: an
/// `untrusted comment:` line, then the key, which is `RW` and 54 more
/// base64 characters, and at most a newline after it.
const fn is_public_key(text: &[u8]) -> bool {
    if text.is_empty() || !text.len().is_multiple_of(4) {
        return false;
    }
    let mut at = 0;
    while at < text.len() {
        let padding = text.len() - at <= 2 && text[at] == b'=';
        if sextet(text[at]) == NOT_BASE64 && !padding {
            return false;
        }
        if text[at] == b'=' && !(padding && (at + 1 == text.len() || text[at + 1] == b'=')) {
            return false;
        }
        at += 1;
    }
    let padded = if text[text.len() - 1] != b'=' {
        0
    } else if text[text.len() - 2] == b'=' {
        2
    } else {
        1
    };
    let decoded = text.len() / 4 * 3 - padded;
    let comment = b"untrusted comment:";
    let mut index = 0;
    while index < comment.len() {
        if index >= decoded || decoded_byte(text, index) != comment[index] {
            return false;
        }
        index += 1;
    }
    while index < decoded && decoded_byte(text, index) != b'\n' {
        index += 1;
    }
    let key = index + 1;
    let mut end = decoded;
    if end > key && decoded_byte(text, end - 1) == b'\n' {
        end -= 1;
    }
    if end < key || end - key != 56 {
        return false;
    }
    if decoded_byte(text, key) != b'R' || decoded_byte(text, key + 1) != b'W' {
        return false;
    }
    let mut index = key;
    while index < end {
        let byte = decoded_byte(text, index);
        if sextet(byte) == NOT_BASE64 && byte != b'=' {
            return false;
        }
        index += 1;
    }
    true
}

const NOT_BASE64: u8 = 64;

/// The value of a standard base64 character, or `NOT_BASE64`.
const fn sextet(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => byte - b'A',
        b'a'..=b'z' => byte - b'a' + 26,
        b'0'..=b'9' => byte - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => NOT_BASE64,
    }
}

/// The byte at `index` of what the valid base64 `text` decodes to.
const fn decoded_byte(text: &[u8], index: usize) -> u8 {
    let group = index / 3 * 4;
    let [a, b, c, d] = [
        bits(text[group]),
        bits(text[group + 1]),
        bits(text[group + 2]),
        bits(text[group + 3]),
    ];
    match index % 3 {
        0 => a << 2 | b >> 4,
        1 => (b & 0b1111) << 4 | c >> 2,
        _ => (c & 0b11) << 6 | d,
    }
}

/// The six bits of a base64 character, with padding as zero bits, which no
/// decoded byte uses.
const fn bits(byte: u8) -> u8 {
    match sextet(byte) {
        NOT_BASE64 => 0,
        value => value,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use axum::Router;
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    use tokio::net::TcpListener;

    use super::{Updates, is_endpoint, is_public_key, is_version};
    use crate::ErrorKind;

    /// A public key, as `cargo packager signer generate` writes it, and the
    /// signature of `signed` under its secret key.
    fn sign(signed: &[u8]) -> (String, String) {
        let keys = minisign::KeyPair::generate_unencrypted_keypair().unwrap();
        let public = STANDARD.encode(keys.pk.to_box().unwrap().to_string());
        let signature =
            minisign::sign(Some(&keys.pk), &keys.sk, Cursor::new(signed), None, None).unwrap();
        (public, STANDARD.encode(signature.to_string()))
    }

    /// Serves version 1.1.0 to a version before it, as the package
    /// `served`, whose signature is `signature`, and returns the endpoint.
    async fn endpoint(served: &'static [u8], signature: String) -> &'static str {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let package = format!("{base}/package.tar.gz");
        let app = Router::new()
            .route(
                "/updates/{current}",
                get(move |Path(current): Path<String>| async move {
                    if current == "1.1.0" {
                        return StatusCode::NO_CONTENT.into_response();
                    }
                    let release = format!(
                        r#"{{"version":"1.1.0","notes":"Faster saves.","url":"{package}","signature":"{signature}","format":"app"}}"#
                    );
                    ([("content-type", "application/json")], release).into_response()
                }),
            )
            .route("/package.tar.gz", get(move || async move { Response::new(axum::body::Body::from(served)) }));
        tokio::spawn(async move { axum::serve(listener, app).await });
        Box::leak(format!("{base}/updates/{{{{current_version}}}}").into_boxed_str())
    }

    fn leak(text: String) -> &'static str {
        Box::leak(text.into_boxed_str())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_newer_version_is_found_and_its_signature_checked() {
        let package: &'static [u8] = b"the new version";
        let (public, signature) = sign(package);
        let endpoint = endpoint(package, signature).await;

        let update = Updates::new("1.0.0", endpoint, leak(public))
            .check()
            .await
            .unwrap()
            .expect("1.1.0 is newer than 1.0.0");
        assert_eq!(update.version(), "1.1.0");
        assert_eq!(update.notes(), Some("Faster saves."));
        update.download().await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_package_its_signature_does_not_match_is_refused() {
        let (public, signature) = sign(b"the new version");
        let endpoint = endpoint(b"something else", signature).await;

        let update = Updates::new("1.0.0", endpoint, leak(public))
            .check()
            .await
            .unwrap()
            .unwrap();
        let error = update.download().await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Update);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn the_newest_version_finds_none() {
        let (public, signature) = sign(b"the new version");
        let endpoint = endpoint(b"the new version", signature).await;

        let update = Updates::new("1.1.0", endpoint, leak(public))
            .check()
            .await
            .unwrap();
        assert!(update.is_none());
    }

    #[test]
    fn a_version_is_semantic() {
        for version in [
            "1.0.0",
            "0.15.2",
            "1.0.0-rc.1",
            "1.0.0-alpha-2+build.7",
            "1.0.0+20261111",
        ] {
            assert!(is_version(version.as_bytes()), "{version}");
        }
        for version in [
            "",
            "1.0",
            "v1.0.0",
            "01.0.0",
            "1.0.0-",
            "1.0.0-01",
            "1.0.0-a..b",
            "1.0.0 ",
            "1.0.0+",
        ] {
            assert!(!is_version(version.as_bytes()), "{version}");
        }
    }

    #[test]
    fn an_endpoint_elsewhere_is_https() {
        for endpoint in [
            "https://updates.example.com/{{target}}",
            "http://127.0.0.1:4000/updates",
            "http://localhost/updates",
        ] {
            assert!(is_endpoint(endpoint.as_bytes()), "{endpoint}");
        }
        for endpoint in [
            "http://updates.example/{{target}}",
            "ftp://127.0.0.1/",
            "http://127.0.0.1.example.com/",
            "http://localhost.example.com/",
            "https://",
            "https:///path",
            "updates.example.com",
        ] {
            assert!(!is_endpoint(endpoint.as_bytes()), "{endpoint}");
        }
    }

    #[test]
    fn a_public_key_is_one_minisign_writes() {
        let (public, _) = sign(b"anything");
        assert!(is_public_key(public.as_bytes()));
        let comment_only = STANDARD.encode("untrusted comment: minisign public key\n");
        let short = STANDARD.encode("untrusted comment: minisign public key\nRWQ=\n");
        for key in [
            "",
            "key",
            "not base64!",
            &comment_only,
            &short,
            &public[1..],
        ] {
            assert!(!is_public_key(key.as_bytes()), "{key}");
        }
    }
}
