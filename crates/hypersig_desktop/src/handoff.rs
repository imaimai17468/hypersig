//! What a second copy of the application hands the copy already running:
//! its command line and its working directory, sent to the running copy's
//! server with the token that copy wrote next to its lock.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Read as _, Write as _};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use tokio::sync::broadcast;

/// The path a second copy posts its launch to, which the guard answers
/// before the application's router sees it.
pub(crate) const PATH: &str = "/_hypersig_desktop/handoff";
/// The header the handoff token travels in.
pub(crate) const HEADER: &str = "hypersig-desktop-handoff";
/// The most a launch may take, which a command line stays well under.
pub(crate) const LIMIT: usize = 1 << 20;

/// How long a second copy keeps trying while the running copy starts.
const PATIENCE: Duration = Duration::from_secs(3);
const RETRY: Duration = Duration::from_millis(100);

/// Another start of the application, which handed its command line to this
/// copy instead of running, from [`Launches::next`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
    dir: PathBuf,
    args: Vec<OsString>,
}

impl Launch {
    pub(crate) fn new(dir: PathBuf, args: Vec<OsString>) -> Self {
        Self { dir, args }
    }

    /// The arguments after the program's name, as `std::env::args_os` gave them.
    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }

    /// The working directory of that start, which a relative path in
    /// [`args`](Self::args) is relative to.
    #[must_use]
    pub fn current_dir(&self) -> &Path {
        &self.dir
    }

    /// `dir`, then each argument, each as its length and its bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for part in
            std::iter::once(self.dir.as_os_str()).chain(self.args.iter().map(OsString::as_os_str))
        {
            let raw = raw(part);
            let len = u32::try_from(raw.len()).unwrap_or(u32::MAX);
            bytes.extend_from_slice(&len.to_le_bytes());
            bytes.extend_from_slice(&raw);
        }
        bytes
    }

    /// The launch [`encode`](Self::encode) wrote, or `None` when `bytes`
    /// is not one.
    pub(crate) fn decode(mut bytes: &[u8]) -> Option<Self> {
        let mut parts = Vec::new();
        while !bytes.is_empty() {
            let (len, rest) = bytes.split_first_chunk::<4>()?;
            let len = usize::try_from(u32::from_le_bytes(*len)).ok()?;
            if rest.len() < len {
                return None;
            }
            let (part, rest) = rest.split_at(len);
            parts.push(cooked(part)?);
            bytes = rest;
        }
        let mut parts = parts.into_iter();
        let dir = PathBuf::from(parts.next()?);
        Some(Self::new(dir, parts.collect()))
    }
}

/// The launches handed to this copy after [`Os::launches`](crate::Os::launches)
/// was called.
#[derive(Debug)]
pub struct Launches {
    receiver: broadcast::Receiver<Launch>,
}

impl Launches {
    pub(crate) fn new(receiver: broadcast::Receiver<Launch>) -> Self {
        Self { receiver }
    }

    /// The next launch, or `None` once the application is closing.
    pub async fn next(&mut self) -> Option<Launch> {
        loop {
            match self.receiver.recv().await {
                Ok(launch) => return Some(launch),
                Err(broadcast::error::RecvError::Lagged(missed)) => {
                    tracing::warn!(
                        missed,
                        "launches arrived faster than they were taken, and the oldest were dropped"
                    );
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }
}

/// The channel every [`Launches`] of one run listens on.
pub(crate) fn channel() -> broadcast::Sender<Launch> {
    broadcast::channel(16).0
}

/// Writes `token` to `dir/handoff`, readable by this user alone on Unix.
pub(crate) fn write_token(dir: &Path, token: &str) -> io::Result<()> {
    let mut options = fs::File::options();
    options.create(true).write(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options
        .open(dir.join("handoff"))?
        .write_all(token.as_bytes())
}

/// Hands this start's command line to the copy running from `dir`, and
/// returns whether that copy took it.
pub(crate) fn hand_over(dir: &Path) -> bool {
    let launch = Launch::new(
        std::env::current_dir().unwrap_or_default(),
        std::env::args_os().skip(1).collect(),
    );
    let body = launch.encode();
    let mut waited = Duration::ZERO;
    loop {
        match send(dir, &body) {
            Ok(()) => return true,
            Err(error) if waited >= PATIENCE => {
                tracing::error!(%error, "the copy already running did not take this start's command line");
                return false;
            }
            Err(_) => {
                thread::sleep(RETRY);
                waited += RETRY;
            }
        }
    }
}

/// Posts `body` to the server whose port and token are recorded in `dir`.
fn send(dir: &Path, body: &[u8]) -> io::Result<()> {
    let port: u16 = fs::read_to_string(dir.join("port"))?
        .trim()
        .parse()
        .map_err(io::Error::other)?;
    let token = fs::read_to_string(dir.join("handoff"))?;
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(1))?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    write!(
        stream,
        "POST {PATH} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n{HEADER}: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        token.trim(),
        body.len()
    )?;
    stream.write_all(body)?;
    let mut answer = [0; 12];
    stream.read_exact(&mut answer)?;
    if answer.ends_with(b" 204") {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "the copy already running answered {}",
            String::from_utf8_lossy(&answer)
        )))
    }
}

/// The bytes of `text` as the system holds them.
#[cfg(unix)]
fn raw(text: &OsStr) -> Vec<u8> {
    std::os::unix::ffi::OsStrExt::as_bytes(text).to_vec()
}

/// The bytes of `text` as the system holds them.
#[cfg(windows)]
fn raw(text: &OsStr) -> Vec<u8> {
    std::os::windows::ffi::OsStrExt::encode_wide(text)
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// The text `raw` wrote `bytes` from.
#[cfg(unix)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "on Windows an odd length is not text"
)]
fn cooked(bytes: &[u8]) -> Option<OsString> {
    Some(std::os::unix::ffi::OsStringExt::from_vec(bytes.to_vec()))
}

/// The text `raw` wrote `bytes` from.
#[cfg(windows)]
fn cooked(bytes: &[u8]) -> Option<OsString> {
    let (pairs, odd) = bytes.as_chunks::<2>();
    if !odd.is_empty() {
        return None;
    }
    let wide: Vec<u16> = pairs.iter().map(|&pair| u16::from_le_bytes(pair)).collect();
    Some(std::os::windows::ffi::OsStringExt::from_wide(&wide))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::PathBuf;

    use super::Launch;

    #[test]
    fn a_launch_comes_back_as_it_was_sent() {
        let launch = Launch::new(
            PathBuf::from("/home/me/notes"),
            vec![
                OsString::from("today.md"),
                OsString::new(),
                OsString::from("日本語.md"),
            ],
        );
        assert_eq!(Launch::decode(&launch.encode()), Some(launch));
    }

    #[test]
    fn bytes_that_are_not_a_launch_are_refused() {
        assert_eq!(Launch::decode(&[]), None);
        assert_eq!(Launch::decode(&[9, 0, 0, 0, b'a']), None);
        assert_eq!(Launch::decode(&[1, 0]), None);
    }
}
