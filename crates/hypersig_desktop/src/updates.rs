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
    #[must_use]
    pub const fn new(
        version: &'static str,
        endpoint: &'static str,
        public_key: &'static str,
    ) -> Self {
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
    /// Fails when the version or the endpoint does not parse, when the
    /// endpoint is not `https`, or plain `http` to this machine, and when the
    /// endpoint cannot be reached or answers with something else.
    pub(crate) async fn check(self) -> Result<Option<Update>, Error> {
        let version = Version::parse(self.version).map_err(Error::update)?;
        let endpoint = Url::parse(self.endpoint).map_err(Error::update)?;
        let local = matches!(endpoint.host_str(), Some("127.0.0.1" | "localhost"));
        if endpoint.scheme() != "https" && !(endpoint.scheme() == "http" && local) {
            return Err(Error::update(
                "the update endpoint is not https, and only a test on this machine may use http",
            ));
        }
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
    /// The version, as the endpoint wrote it.
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
    /// Installs the package over the running application, which runs the
    /// new version once it is started again.
    ///
    /// On Windows the installer takes over and the application exits; on
    /// macOS the `.app` bundle and on Linux the `AppImage` are replaced.
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

    use super::Updates;
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

    #[tokio::test]
    async fn an_endpoint_elsewhere_must_be_https() {
        for endpoint in ["http://updates.example/{{target}}", "ftp://127.0.0.1/"] {
            let error = Updates::new("1.0.0", endpoint, "key")
                .check()
                .await
                .unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Update, "{endpoint}");
        }
    }
}
