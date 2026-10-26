//! Serving a [`Router`] on the address the environment names, until the
//! process is told to stop.

use std::io;
use std::net::{Ipv4Addr, SocketAddr};
use std::pin::pin;
use std::time::Duration;

use axum::Router;
use tokio::net::TcpListener;

/// How long the server waits for the requests in progress after a signal.
///
/// A stream that a page holds open never finishes on its own, so without a
/// limit one open tab would keep the server from stopping. Five seconds is
/// within the 10 seconds `docker stop` waits before it kills the container.
const GRACE: Duration = Duration::from_secs(5);

/// Serves `app` on the address in `ADDR` or `PORT` until Ctrl+C or SIGTERM.
///
/// The address is the one in `ADDR` where it is set, such as
/// `127.0.0.1:3100`. Otherwise, where `PORT` is set, it is that port on every
/// interface, because a container host names the port in `PORT` and connects
/// from outside the container. With neither, it is `port` on `127.0.0.1`,
/// which only this machine reaches. The function prints the address to
/// standard output once it listens.
///
/// On Ctrl+C or SIGTERM the server accepts no new connection and waits for
/// the requests in progress. A stream a page keeps open does not finish by
/// itself, so the wait ends after five seconds, or at a second Ctrl+C or
/// SIGTERM, and the connections still open are closed then.
///
/// # Errors
///
/// Returns an error of kind [`io::ErrorKind::InvalidInput`] when `ADDR` is
/// not a socket address or `PORT` is not a port number, and the error of
/// binding the address or of serving a connection otherwise.
///
/// # Examples
///
/// ```no_run
/// use axum::Router;
/// use hypersig::{Page, Route, RouterExt, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Home {}
///
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const INDEX: Route = "/" => index;
/// }
///
/// async fn index((): (), (): ()) -> Page<Home> {
///     Page::new("Home", Home {})
/// }
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     // `127.0.0.1:3000`, unless `ADDR` or `PORT` names another address.
///     hypersig::serve(pages(Router::new()).hypersig_runtime(), 3000).await
/// }
/// ```
#[expect(
    clippy::print_stdout,
    reason = "an application calls `serve` from `main`, where the address it listens on is what the person starting it reads"
)]
pub async fn serve(app: Router, port: u16) -> io::Result<()> {
    let addr = address(
        std::env::var("ADDR").ok().as_deref(),
        std::env::var("PORT").ok().as_deref(),
        port,
    )?;
    let listener = TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;
    if bound.ip().is_unspecified() {
        println!("Listening on http://localhost:{}", bound.port());
    } else {
        println!("Listening on http://{bound}");
    }

    let mut signals = Signals::new()?;
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let mut server = pin!(
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .into_future()
    );
    tokio::select! {
        result = &mut server => return result,
        () = signals.recv() => {}
    }
    let _ = stop.send(());
    tokio::select! {
        result = &mut server => result,
        () = signals.recv() => Ok(()),
        () = tokio::time::sleep(GRACE) => Ok(()),
    }
}

/// The address to listen on, from the values of `ADDR` and `PORT` and the
/// port the application chose.
fn address(addr: Option<&str>, port_var: Option<&str>, port: u16) -> io::Result<SocketAddr> {
    if let Some(addr) = addr {
        return addr.parse().map_err(|error| {
            invalid(format!(
                "ADDR is `{addr}`, which is not a socket address such as 127.0.0.1:3000: {error}"
            ))
        });
    }
    if let Some(port) = port_var {
        let port: u16 = port.parse().map_err(|error| {
            invalid(format!(
                "PORT is `{port}`, which is not a port number from 0 to 65535: {error}"
            ))
        })?;
        return Ok(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)));
    }
    Ok(SocketAddr::from((Ipv4Addr::LOCALHOST, port)))
}

fn invalid(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

/// The signals that stop the server: Ctrl+C, and on Unix the SIGTERM a
/// container host sends. A server that runs as a container's first process
/// is not stopped by the kernel on a signal it does not handle.
struct Signals {
    #[cfg(unix)]
    terminate: tokio::signal::unix::Signal,
}

impl Signals {
    fn new() -> io::Result<Self> {
        Ok(Self {
            #[cfg(unix)]
            terminate: tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?,
        })
    }

    /// Waits for the next signal.
    async fn recv(&mut self) {
        let ctrl_c = async {
            // Where Ctrl+C cannot be watched, only the other signal stops
            // the server, rather than the failure stopping it at once.
            if tokio::signal::ctrl_c().await.is_err() {
                std::future::pending::<()>().await;
            }
        };
        #[cfg(unix)]
        tokio::select! {
            () = ctrl_c => {}
            _ = self.terminate.recv() => {}
        }
        #[cfg(not(unix))]
        ctrl_c.await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_is_used_as_written() {
        let addr = address(Some("127.0.0.1:3100"), Some("8080"), 3000).unwrap();
        assert_eq!(addr, "127.0.0.1:3100".parse().unwrap());
    }

    #[test]
    fn port_listens_on_every_interface() {
        let addr = address(None, Some("8080"), 3000).unwrap();
        assert_eq!(addr, "0.0.0.0:8080".parse().unwrap());
    }

    #[test]
    fn neither_listens_on_this_machine_at_the_given_port() {
        let addr = address(None, None, 3000).unwrap();
        assert_eq!(addr, "127.0.0.1:3000".parse().unwrap());
    }

    #[test]
    fn a_malformed_value_names_the_variable() {
        let error = address(Some("localhost"), None, 3000).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains("ADDR"));
        let error = address(None, Some("http"), 3000).unwrap_err();
        assert!(error.to_string().contains("PORT"));
    }
}
