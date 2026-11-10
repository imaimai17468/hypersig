//! The port the server listens on, kept from one run of the application to the next.

use std::fs;
use std::io;
use std::net::Ipv4Addr;
use std::path::Path;

use tokio::net::TcpListener;

/// Listens on `127.0.0.1` at the port recorded in `dir`, or at a port the
/// system picks when there is no record or another program holds that port,
/// which is then recorded.
///
/// The port is part of the page's origin, and the browser keeps
/// `localStorage` per origin, so a new port leaves the signals a page
/// persisted behind.
pub(crate) async fn listen(dir: &Path) -> io::Result<TcpListener> {
    let record = dir.join("port");
    let kept = fs::read_to_string(&record)
        .ok()
        .and_then(|text| text.trim().parse::<u16>().ok())
        .filter(|&port| port != 0);
    if let Some(port) = kept {
        match TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await {
            Ok(listener) => return Ok(listener),
            Err(error) => tracing::warn!(
                port,
                %error,
                "the port this application listened on before is taken, so the signals its pages persisted start over"
            ),
        }
    }
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await?;
    fs::write(&record, listener.local_addr()?.port().to_string())?;
    Ok(listener)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::net::Ipv4Addr;

    use super::listen;

    #[tokio::test]
    async fn the_port_is_kept_and_replaced_when_taken() {
        let dir = env::temp_dir().join(format!("hypersig_desktop-port-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();

        let first = listen(&dir).await.unwrap();
        let port = first.local_addr().unwrap().port();
        assert_eq!(
            fs::read_to_string(dir.join("port")).unwrap(),
            port.to_string()
        );
        drop(first);

        let again = listen(&dir).await.unwrap();
        assert_eq!(again.local_addr().unwrap().port(), port);

        // `again` still holds the port, as another program would.
        let moved = listen(&dir).await.unwrap();
        let new_port = moved.local_addr().unwrap().port();
        assert_ne!(new_port, port);
        assert_eq!(
            fs::read_to_string(dir.join("port")).unwrap(),
            new_port.to_string()
        );
        assert_eq!(moved.local_addr().unwrap().ip(), Ipv4Addr::LOCALHOST);

        drop((again, moved));
        fs::remove_dir_all(&dir).unwrap();
    }
}
