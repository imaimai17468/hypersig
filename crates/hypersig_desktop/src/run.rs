//! Starting the server and the window side by side, and stopping both.

use std::env;
use std::fs;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use hypersig::Url;
use tao::event::{Event as TaoEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::platform::run_return::EventLoopExtRunReturn as _;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use crate::app::{App, IntoRouter};
use crate::error::Error;
use crate::guard::{self, Guard};
use crate::{instance, port, window};

/// How long the server is given to finish the requests it is answering
/// once the window closes, the same as `hypersig::serve` gives.
const GRACE: Duration = Duration::from_secs(5);

/// How long the hidden window waits for its first page before it is shown anyway.
const FIRST_LOAD: Duration = Duration::from_secs(10);

/// The environment variable that makes the application close once its first page loads.
const SMOKE: &str = "HYPERSIG_DESKTOP_SMOKE";

/// What the server's tasks and the web view tell the event loop.
#[derive(Debug)]
pub(crate) enum Event {
    /// The router is built and the server is listening.
    Ready,
    /// The setup failed, or the server stopped on an error.
    Failed(Error),
    /// The page's title changed.
    Title(String),
    /// A page of the application finished loading.
    Loaded,
    /// The first page has not finished loading in [`FIRST_LOAD`].
    Slow,
    /// Ctrl-C was pressed in the terminal that started the application.
    Quit,
}

impl App {
    /// Serves the router `setup` builds and opens `first_page` in a window,
    /// until the window closes.
    ///
    /// The window and the web view are made while `setup` runs, on a Tokio
    /// runtime this method starts, and the window is shown once the page has
    /// loaded. Closing the window, or Ctrl-C in the terminal, stops the
    /// server, which is given five seconds to finish the requests it is
    /// answering.
    ///
    /// The window and its event loop take the thread that calls this, which
    /// on macOS must be the main thread, so `main` is not
    /// `#[tokio::main]`.
    ///
    /// # Errors
    ///
    /// Fails when the application's directory cannot be made, when another
    /// copy of the application runs, when `setup` returns an error, and when
    /// the window, the web view, or the server fails.
    ///
    /// # Panics
    ///
    /// Panics on macOS when it is not called on the main thread.
    pub fn run<F>(self, first_page: &Url, setup: F) -> Result<(), Error>
    where
        F: Future<Output: IntoRouter> + Send + 'static,
    {
        let started = Instant::now();
        let own = self.data_dir()?.join("hypersig_desktop");
        fs::create_dir_all(&own)?;
        let lock = instance::lock(&own)?;

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let listener = runtime.block_on(port::listen(&own))?;
        let port = listener.local_addr()?.port();
        let origin = format!("http://127.0.0.1:{port}/");
        let (guard, start) = Guard::new(port, first_page.as_str())?;

        let mut event_loop = EventLoopBuilder::<Event>::with_user_event().build();
        let proxy = event_loop.create_proxy();
        let (stop, stopped) = oneshot::channel();
        let server = runtime.spawn(serve(listener, guard, setup, proxy.clone(), stopped));
        runtime.spawn(quit_on_ctrl_c(proxy.clone()));

        let window = window::window(&self, &event_loop)?;
        let mut context = window::context(own.join("webview"));
        let web_view = window::web_view(&window, &mut context, origin, &proxy)?;

        let smoke = env::var_os(SMOKE).is_some();
        let mut shown = false;
        let mut failure = None;
        event_loop.run_return(|event, _, flow| {
            *flow = ControlFlow::Wait;
            match event {
                TaoEvent::UserEvent(Event::Ready) => {
                    if let Err(error) = web_view.load_url(&start) {
                        failure = Some(Error::window(error));
                        *flow = ControlFlow::Exit;
                    }
                }
                TaoEvent::UserEvent(Event::Failed(error)) => {
                    failure = Some(error);
                    *flow = ControlFlow::Exit;
                }
                TaoEvent::UserEvent(Event::Title(title)) => window.set_title(&title),
                TaoEvent::UserEvent(Event::Loaded) if !shown => {
                    shown = true;
                    window.set_visible(true);
                    window.set_focus();
                    if smoke {
                        report_start(started.elapsed());
                        *flow = ControlFlow::Exit;
                    }
                }
                TaoEvent::UserEvent(Event::Slow) if !shown => {
                    shown = true;
                    tracing::error!(
                        "the first page did not finish loading in {} seconds, so the window is shown without it",
                        FIRST_LOAD.as_secs()
                    );
                    window.set_visible(true);
                    if smoke {
                        failure = Some(Error::window(io::Error::from(io::ErrorKind::TimedOut)));
                        *flow = ControlFlow::Exit;
                    }
                }
                TaoEvent::UserEvent(Event::Quit)
                | TaoEvent::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *flow = ControlFlow::Exit,
                _ => {}
            }
        });

        // The web view ends its requests and streams as it goes, so the
        // server has none left open when it is asked to stop.
        drop(web_view);
        drop(window);
        let _ = stop.send(());
        runtime.block_on(async {
            let _ = tokio::time::timeout(GRACE, server).await;
        });
        runtime.shutdown_timeout(GRACE);
        drop(lock);
        failure.map_or(Ok(()), Err)
    }
}

/// Builds the router, tells the event loop that the server is ready, and
/// serves until `stopped` resolves.
async fn serve<F>(
    listener: TcpListener,
    guard: Arc<Guard>,
    setup: F,
    proxy: EventLoopProxy<Event>,
    stopped: oneshot::Receiver<()>,
) where
    F: Future<Output: IntoRouter>,
{
    let router = match setup.await.into_router() {
        Ok(router) => router,
        Err(error) => {
            let _ = proxy.send_event(Event::Failed(Error::setup(error)));
            return;
        }
    };
    let app = guard::wrap(router, guard);
    let _ = proxy.send_event(Event::Ready);
    let slow = proxy.clone();
    tokio::spawn(async move {
        tokio::time::sleep(FIRST_LOAD).await;
        let _ = slow.send_event(Event::Slow);
    });
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = stopped.await;
        })
        .await;
    if let Err(error) = served {
        let _ = proxy.send_event(Event::Failed(error.into()));
    }
}

async fn quit_on_ctrl_c(proxy: EventLoopProxy<Event>) {
    if tokio::signal::ctrl_c().await.is_ok() {
        let _ = proxy.send_event(Event::Quit);
    }
}

#[expect(
    clippy::print_stdout,
    reason = "the measurement is read from standard output by whatever started the application"
)]
fn report_start(elapsed: Duration) {
    println!(
        "hypersig_desktop: first page shown {} ms after run",
        elapsed.as_millis()
    );
}
