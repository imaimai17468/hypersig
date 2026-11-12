//! Starting the server and the windows side by side, and stopping both.

use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::http::HeaderMap;
use hypersig::Url;
use tao::event::{Event as TaoEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
use tao::platform::run_return::EventLoopExtRunReturn as _;
use tao::window::{Window, WindowId};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use wry::{WebContext, WebView};

use crate::app::{App, IntoRouter};
use crate::error::Error;
use crate::guard::{self, Guard};
use crate::os::Os;
use crate::smoke::{Smoke, Step};
use crate::{handoff, instance, port, window};

/// How long the server is given to finish the requests it is answering
/// once the last window closes, the same as `hypersig::serve` gives.
const GRACE: Duration = Duration::from_secs(5);

/// How long a hidden window waits for its page before it is shown anyway.
const FIRST_LOAD: Duration = Duration::from_secs(10);

/// How long a closing window stays, hidden, after its page leaves. The web
/// view writes `localStorage` to disk a moment after a page sets it, and a
/// value set just before the window closed was lost when the process ended
/// as soon as the page had left; with this wait, it was kept in each of
/// four runs in `WKWebView`.
const LEAVE: Duration = Duration::from_secs(1);

/// What the server's tasks, the web views, and [`Os`] tell the event loop.
#[derive(Debug)]
pub(crate) enum Event {
    /// The setup failed, or the server stopped on an error.
    Failed(Error),
    /// The title of a window's page changed.
    Title(WindowId, String),
    /// A page of the application finished loading in a window.
    Loaded(WindowId),
    /// A window's page has not finished loading in [`FIRST_LOAD`].
    Slow(WindowId),
    /// A page asks to be opened in a new window, answered on `answer` with
    /// the window, when [`Os`] asked.
    Open {
        url: String,
        answer: Option<oneshot::Sender<Result<WindowId, Error>>>,
    },
    /// [`Os`] shows a tray icon, or replaces the one shown.
    Tray {
        tray: crate::Tray,
        answer: oneshot::Sender<Result<(), Error>>,
    },
    /// [`Os`] registers a global shortcut, or listens to one registered.
    Shortcut {
        shortcut: crate::Shortcut,
        answer: oneshot::Sender<Result<crate::Presses, Error>>,
    },
    /// A global shortcut went down or came up.
    Pressed(u32, crate::Press),
    /// An item of the menu bar or of the tray's menu was chosen.
    Menu(tray_icon::menu::MenuId),
    /// [`Os`] closes a window.
    Close(WindowId),
    /// [`Os`] brings a window to the front.
    Focus(WindowId),
    /// [`Os`] asks which window had the keyboard last.
    Focused(oneshot::Sender<Option<WindowId>>),
    /// [`Os`] moves, sizes, or places a window.
    Arrange(WindowId, crate::arrange::Arrange),
    /// The count to show on the application's icon.
    Badge(u32),
    /// [`LEAVE`] has passed since a window began to close, or, with no
    /// window, the application is quitting.
    Left(Option<WindowId>),
    /// An update closes the application and installs its package after
    /// the server has stopped.
    #[cfg(feature = "updater")]
    Install(Box<crate::Downloaded>),
    /// Ctrl-C was pressed in the terminal, or Quit was chosen in the menu.
    Quit,
    /// A second copy handed its command line over.
    Raise,
    /// The first page's answer to what a smoke run asked, or `None` when
    /// it is time to ask again.
    Smoke(Option<String>),
}

#[cfg(feature = "updater")]
impl App<crate::Updates> {
    /// Serves the router `setup` builds and shows `first_page` in a window,
    /// as [`App::run`](App::<()>::run) does.
    ///
    /// `setup` receives the [`Os`] and the [`Updater`](crate::Updater) that
    /// finds new versions where [`App::updates`] says.
    ///
    /// # Errors
    ///
    /// Fails where [`App::run`](App::<()>::run) fails.
    ///
    /// # Panics
    ///
    /// Panics on macOS when it is not called on the main thread.
    pub fn run<S, F>(self, first_page: &Url, setup: S) -> Result<(), Error>
    where
        S: FnOnce(Os, crate::Updater) -> F,
        F: Future<Output: IntoRouter> + Send + 'static,
    {
        let updater = crate::Updater::system(self.updates);
        let app = App {
            id: self.id,
            size: self.size,
            min_size: self.min_size,
            resizable: self.resizable,
            devices: self.devices,
            updates: (),
        };
        app.run(first_page, move |os| setup(os, updater))
    }
}

impl App {
    /// Serves the router `setup` builds and shows `first_page` in a window.
    ///
    /// `setup` receives the [`Os`] the router's handlers open windows and
    /// reach the operating system through, and returns the future that
    /// builds the router. That future runs on a Tokio runtime this method
    /// starts, while the first window and its web view are made, and the
    /// window is shown once its page has loaded.
    ///
    /// When another copy of the application runs, this start opens no
    /// window: it hands its command line to that copy, which hears of it
    /// through [`Os::launches`] and brings a window forward, and returns
    /// `Ok(())`.
    ///
    /// Closing the last window while no tray is shown, Quit in the menu or
    /// in the tray's menu, or Ctrl-C in the terminal stops the server,
    /// which is given five seconds to finish the requests it is answering,
    /// and the method returns once it has stopped.
    ///
    /// The windows and their event loop take the thread that calls this,
    /// which on macOS must be the main thread, so `main` is not
    /// `#[tokio::main]`.
    ///
    /// # Errors
    ///
    /// Fails with [`ErrorKind::InRuntime`](crate::ErrorKind::InRuntime) when
    /// called inside a Tokio runtime, as from a `#[tokio::main]` function.
    /// Fails too when the application's directory cannot be made, when another
    /// copy of the application runs and does not take this start's command
    /// line, when the setup returns an error, and when a window, a web view,
    /// or the server fails.
    ///
    /// # Panics
    ///
    /// Panics on macOS when it is not called on the main thread.
    pub fn run<S, F>(self, first_page: &Url, setup: S) -> Result<(), Error>
    where
        S: FnOnce(Os) -> F,
        F: Future<Output: IntoRouter> + Send + 'static,
    {
        // Inside a runtime, `block_on` below would panic with a message
        // that names neither `run` nor `#[tokio::main]`.
        if tokio::runtime::Handle::try_current().is_ok() {
            return Err(Error::in_runtime());
        }
        let started = Instant::now();
        #[cfg(target_os = "macos")]
        crate::notification::name_application(self.id);
        let own = self.data_dir()?.join("hypersig_desktop");
        fs::create_dir_all(&own)?;
        let lock = match instance::lock(&own) {
            // The copy already running takes this start's command line, and
            // brings a window forward.
            Err(error) if error.kind() == crate::ErrorKind::AlreadyRunning => {
                return if handoff::hand_over(&own) {
                    Ok(())
                } else {
                    Err(error)
                };
            }
            locked => locked?,
        };

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let listener = runtime.block_on(port::listen(&own))?;
        let port = listener.local_addr()?.port();
        let launches = handoff::channel();
        let (guard, tokens) = Guard::new(port, launches.clone())?;
        handoff::write_token(&own, &tokens.handoff)?;
        let start = tokens.start;

        let mut event_loop = EventLoopBuilder::<Event>::with_user_event().build();
        let proxy = event_loop.create_proxy();
        let (stop, stopped) = oneshot::channel();
        let setup = setup(Os::new(proxy.clone(), launches.clone()));
        runtime.spawn(raise_on_launch(launches.subscribe(), proxy.clone()));
        let server = runtime.spawn(serve(listener, guard, setup, proxy.clone(), stopped));
        runtime.spawn(quit_on_ctrl_c(proxy.clone()));

        #[cfg(target_os = "macos")]
        let _menu = crate::menu::install()?;
        let chosen = proxy.clone();
        tray_icon::menu::MenuEvent::set_event_handler(Some(
            move |event: tray_icon::menu::MenuEvent| {
                let _ = chosen.send_event(Event::Menu(event.id));
            },
        ));
        let pressed = proxy.clone();
        global_hotkey::GlobalHotKeyEvent::set_event_handler(Some(
            move |event: global_hotkey::GlobalHotKeyEvent| {
                let _ = pressed.send_event(Event::Pressed(event.id(), event.state().into()));
            },
        ));
        #[cfg(feature = "updater")]
        crate::updates::event_loop(Some(proxy.clone()));
        let mut run = Lifecycle {
            app: self,
            proxy: &proxy,
            runtime: &runtime,
            context: window::context(own.join("webview")),
            base: format!("http://127.0.0.1:{port}"),
            windows: HashMap::new(),
            first: None,
            focused: None,
            waiting: Vec::new(),
            started: false,
            smoke: Smoke::from_env(started)?,
            badge: 0,
            tray: None,
            hotkeys: None,
            #[cfg(feature = "updater")]
            install: None,
            shortcuts: HashMap::new(),
            items: HashMap::new(),
            quitting: false,
            failure: None,
        };
        // The first page is asked for at once, while the event loop starts
        // and the setup runs: the listener holds the request until the
        // server answers it.
        let mut headers = HeaderMap::new();
        headers.insert(guard::START_HEADER, start);
        let first = run.open(&event_loop, Some((first_page.as_str(), headers)))?;
        run.first = Some(first);
        event_loop.run_return(|event, target, flow| {
            *flow = if run.handle(event, target) {
                ControlFlow::Exit
            } else {
                ControlFlow::Wait
            };
        });
        let failure = run.failure.take();
        #[cfg(feature = "updater")]
        let install = run.install.take();
        #[cfg(feature = "updater")]
        crate::updates::event_loop(None);
        drop(run);

        let _ = stop.send(());
        runtime.block_on(async {
            let _ = tokio::time::timeout(GRACE, server).await;
        });
        runtime.shutdown_timeout(GRACE);
        drop(lock);
        failure.map_or(Ok(()), Err)?;
        #[cfg(feature = "updater")]
        install_after_stop(install)?;
        Ok(())
    }
}

/// A window, its web view, and how far its page has come.
struct Open {
    window: Window,
    web_view: WebView,
    shown: bool,
    leaving: bool,
}

/// A window a page or [`Os`] asked for, with where to answer.
struct Asked {
    url: String,
    answer: Option<oneshot::Sender<Result<WindowId, Error>>>,
}

/// The windows and how far the run has come.
struct Lifecycle<'a> {
    app: App,
    proxy: &'a EventLoopProxy<Event>,
    runtime: &'a Runtime,
    /// The web view data every window shares, cookies included.
    context: WebContext,
    /// `http://127.0.0.1:<port>`, which a page's path is joined to.
    base: String,
    windows: HashMap<WindowId, Open>,
    /// The first window, whose first page trades the start token for the
    /// session cookie.
    first: Option<WindowId>,
    /// The window that had the keyboard last.
    focused: Option<WindowId>,
    /// The windows made before the first page loaded, with the page each
    /// loads once it has, since a page needs the cookie the first one
    /// receives.
    waiting: Vec<(WindowId, String)>,
    /// Whether the first page has loaded.
    started: bool,
    smoke: Option<Smoke>,
    /// The tray icon shown, which keeps the application running with no window.
    tray: Option<tray_icon::TrayIcon>,
    /// The package an update installs once the server has stopped.
    #[cfg(feature = "updater")]
    install: Option<crate::Downloaded>,
    /// What registers the global shortcuts, made on this thread when the
    /// first one is asked for, as macOS and Windows need.
    hotkeys: Option<global_hotkey::GlobalHotKeyManager>,
    /// Where each global shortcut, by its id, reports its presses.
    shortcuts: HashMap<u32, tokio::sync::broadcast::Sender<crate::Press>>,
    /// Where each item of the tray's menu reports its clicks.
    items: HashMap<tray_icon::menu::MenuId, tokio::sync::broadcast::Sender<()>>,
    /// The count on the application's icon, which a window opened later
    /// shows too on Windows.
    badge: u32,
    quitting: bool,
    failure: Option<Error>,
}

impl Lifecycle<'_> {
    /// Answers one event, and returns whether the event loop is done.
    fn handle(
        &mut self,
        event: TaoEvent<'_, Event>,
        target: &EventLoopWindowTarget<Event>,
    ) -> bool {
        match event {
            TaoEvent::UserEvent(Event::Failed(error)) => self.fail(error),
            TaoEvent::UserEvent(Event::Title(id, title)) => {
                if let Some(open) = self.windows.get(&id) {
                    open.window.set_title(&title);
                }
            }
            TaoEvent::UserEvent(Event::Loaded(id)) => self.loaded(id),
            TaoEvent::UserEvent(Event::Slow(id)) => self.slow(id),
            TaoEvent::UserEvent(Event::Open { url, answer }) => {
                if self.started {
                    self.answer(target, Asked { url, answer });
                } else {
                    self.wait(target, Asked { url, answer });
                }
            }
            TaoEvent::UserEvent(Event::Close(id))
            | TaoEvent::WindowEvent {
                window_id: id,
                event: WindowEvent::CloseRequested,
                ..
            } => self.leave(id),
            TaoEvent::UserEvent(Event::Badge(count)) => {
                self.badge = count;
                let windows = self.windows.values().filter(|open| !open.leaving);
                crate::badge::show(count, windows.map(|open| &open.window), target);
            }
            TaoEvent::UserEvent(Event::Focus(id)) => {
                if let Some(open) = self.windows.get(&id) {
                    open.window.set_focus();
                }
            }
            TaoEvent::WindowEvent {
                window_id: id,
                event: WindowEvent::Focused(true),
                ..
            } => self.focused = Some(id),
            TaoEvent::UserEvent(Event::Focused(answer)) => {
                let open = self
                    .focused
                    .filter(|id| self.windows.get(id).is_some_and(|open| !open.leaving));
                let _ = answer.send(open);
            }
            TaoEvent::UserEvent(Event::Arrange(id, change)) => {
                if let Some(open) = self.windows.get(&id) {
                    crate::arrange::apply(&open.window, change);
                }
            }
            TaoEvent::UserEvent(Event::Left(id)) => {
                if let Some(id) = id
                    && self.windows.get(&id).is_some_and(|open| open.leaving)
                {
                    self.windows.remove(&id);
                }
                // A tray keeps the application running with no window,
                // until its Quit item or another way to quit is chosen.
                return self.windows.is_empty() && (self.tray.is_none() || self.quitting);
            }
            TaoEvent::UserEvent(Event::Quit) => self.quit(),
            #[cfg(feature = "updater")]
            TaoEvent::UserEvent(Event::Install(downloaded)) => {
                self.install = Some(*downloaded);
                self.quit();
            }
            TaoEvent::UserEvent(Event::Tray { tray, answer }) => {
                let _ = answer.send(self.tray(&tray));
            }
            TaoEvent::UserEvent(Event::Menu(id)) => self.chosen(&id),
            TaoEvent::UserEvent(Event::Shortcut { shortcut, answer }) => {
                let _ = answer.send(self.shortcut(shortcut));
            }
            TaoEvent::UserEvent(Event::Pressed(id, press)) => {
                if let Some(presses) = self.shortcuts.get(&id) {
                    let _ = presses.send(press);
                }
            }
            TaoEvent::UserEvent(Event::Raise) => self.raise(),
            TaoEvent::UserEvent(Event::Smoke(answer)) => self.smoke(answer.as_deref()),
            _ => {}
        }
        false
    }

    /// Makes a hidden window with its web view, which loads `page`, a path
    /// of the application or a URL of its origin, with the headers given.
    fn open(
        &mut self,
        target: &EventLoopWindowTarget<Event>,
        page: Option<(&str, HeaderMap)>,
    ) -> Result<WindowId, Error> {
        if self.quitting {
            return Err(Error::closed());
        }
        let window = window::window(&self.app, target)?;
        #[cfg(windows)]
        crate::badge::on_taskbar(&window, self.badge);
        let origin = format!("{}/", self.base);
        let web_view =
            window::web_view(&self.app, &window, &mut self.context, &origin, self.proxy)?;
        let id = window.id();
        if let Some((url, headers)) = page {
            self.load(id, &web_view, url, headers)?;
        }
        self.windows.insert(
            id,
            Open {
                window,
                web_view,
                shown: false,
                leaving: false,
            },
        );
        Ok(id)
    }

    /// Has `web_view`, of the window `id`, load `url`, a path of the
    /// application or a URL of its origin, and shows the window without its
    /// page if the page has not loaded after [`FIRST_LOAD`].
    fn load(
        &self,
        id: WindowId,
        web_view: &WebView,
        url: &str,
        headers: HeaderMap,
    ) -> Result<(), Error> {
        let absolute = if url.starts_with('/') {
            format!("{}{url}", self.base)
        } else {
            url.to_owned()
        };
        web_view
            .load_url_with_headers(&absolute, headers)
            .map_err(Error::window)?;
        let slow = self.proxy.clone();
        self.runtime.spawn(async move {
            tokio::time::sleep(FIRST_LOAD).await;
            let _ = slow.send_event(Event::Slow(id));
        });
        Ok(())
    }

    /// Makes the window `asked` for before the first page has loaded and
    /// tells whoever asked, so a setup that awaits the window can return
    /// the router the first page waits for. Its page loads once the first
    /// page has.
    fn wait(&mut self, target: &EventLoopWindowTarget<Event>, Asked { url, answer }: Asked) {
        let made = self.open(target, None);
        match &made {
            Ok(id) => self.waiting.push((*id, url)),
            Err(error) => tracing::error!(%error, %url, "a window could not be opened"),
        }
        if let Some(answer) = answer {
            let _ = answer.send(made);
        }
    }

    fn loaded(&mut self, id: WindowId) {
        let Some(open) = self.windows.get_mut(&id) else {
            return;
        };
        if open.leaving {
            return;
        }
        // `slow` shows a window whose page is late, and the page can still
        // load after that.
        let late = open.shown;
        if !late {
            open.shown = true;
            open.window.set_visible(true);
            open.window.set_focus();
            // The system may not report the focus of a window shown while
            // another application is in front.
            self.focused = Some(id);
        }
        if self.first == Some(id) {
            if !late {
                self.smoke(None);
            }
            // The first page brings the cookie the other windows need, so
            // they open once it has loaded, late or not.
            if !self.started {
                self.started();
            }
        } else if !late
            && let Some(smoke) = &self.smoke
            && let Err(error) = smoke.other_window(&open.web_view)
        {
            self.fail(error);
        }
    }

    /// Loads the pages of the windows made while the first page loaded,
    /// now that the cookie they need is there.
    fn started(&mut self) {
        self.started = true;
        for (id, url) in std::mem::take(&mut self.waiting) {
            let Some(open) = self.windows.get(&id).filter(|open| !open.leaving) else {
                continue;
            };
            if let Err(error) = self.load(id, &open.web_view, &url, HeaderMap::new()) {
                tracing::error!(%error, %url, "a window could not load its page");
                self.leave(id);
            }
        }
    }

    /// Opens the window `asked` for, and tells whoever asked.
    fn answer(&mut self, target: &EventLoopWindowTarget<Event>, Asked { url, answer }: Asked) {
        let opened = self.open(target, Some((&url, HeaderMap::new())));
        if let Err(error) = &opened {
            tracing::error!(%error, %url, "a window could not be opened");
        }
        if let Some(answer) = answer {
            let _ = answer.send(opened);
        }
    }

    fn slow(&mut self, id: WindowId) {
        let Some(open) = self.windows.get_mut(&id) else {
            return;
        };
        if open.shown || open.leaving {
            return;
        }
        tracing::error!(
            "a page did not finish loading in {} seconds, so its window is shown without it",
            FIRST_LOAD.as_secs()
        );
        open.shown = true;
        open.window.set_visible(true);
        // The windows made meanwhile would load their pages without the
        // cookie, so they close instead.
        if self.first == Some(id) {
            for (waiting, url) in std::mem::take(&mut self.waiting) {
                tracing::error!(%url, "a window was closed, since the first page did not load");
                self.leave(waiting);
            }
        }
        if self.smoke.is_some() {
            self.fail(Error::smoke("a page did not finish loading in time"));
        }
    }

    /// Brings a shown window forward, the first one while it is open.
    fn raise(&self) {
        let shown = |open: &&Open| open.shown && !open.leaving;
        let window = self
            .first
            .and_then(|id| self.windows.get(&id))
            .filter(shown)
            .or_else(|| self.windows.values().find(shown));
        if let Some(open) = window {
            open.window.set_focus();
        }
    }

    /// Takes the smoke run's next step in the first window, if this is one.
    fn smoke(&mut self, answer: Option<&str>) {
        let Some(smoke) = &mut self.smoke else {
            return;
        };
        let Some(open) = self.first.and_then(|id| self.windows.get(&id)) else {
            return;
        };
        if open.leaving {
            return;
        }
        match smoke.step(answer, &open.web_view, self.proxy, self.runtime) {
            Ok(Step::Waiting) => {}
            // A window opened at start can load its page before the first
            // page has answered, and `loaded` ran the script in none then.
            Ok(Step::Scripting) => {
                let first = self.first;
                let ran = self
                    .windows
                    .iter()
                    .filter(|(id, open)| Some(**id) != first && open.shown && !open.leaving)
                    .try_for_each(|(_, open)| smoke.other_window(&open.web_view));
                if let Err(error) = ran {
                    self.fail(error);
                }
            }
            Ok(Step::Done) => self.quit(),
            Err(error) => self.fail(error),
        }
    }

    /// Keeps the first failure and closes every window.
    fn fail(&mut self, error: Error) {
        self.failure.get_or_insert(error);
        self.quit();
    }

    fn quit(&mut self) {
        self.quitting = true;
        let ids: Vec<WindowId> = self.windows.keys().copied().collect();
        if ids.is_empty() {
            // Only a tray kept the application running, so no window's
            // `Left` will end the event loop.
            let _ = self.proxy.send_event(Event::Left(None));
        }
        for id in ids {
            self.leave(id);
        }
    }

    /// Shows `tray`, in place of the tray shown before.
    fn tray(&mut self, tray: &crate::Tray) -> Result<(), Error> {
        let icon = crate::tray::show(tray)?;
        self.items = tray
            .items
            .iter()
            .map(|item| (item.id.clone(), item.clicked.clone()))
            .collect();
        self.tray = Some(icon);
        Ok(())
    }

    /// Registers `shortcut` for the rest of the run, or listens to it again
    /// when it is registered already.
    fn shortcut(&mut self, shortcut: crate::Shortcut) -> Result<crate::Presses, Error> {
        let hotkey = shortcut.hotkey();
        if let Some(presses) = self.shortcuts.get(&hotkey.id()) {
            return Ok(crate::Presses::new(presses.subscribe()));
        }
        let hotkeys = match &mut self.hotkeys {
            Some(hotkeys) => hotkeys,
            empty => {
                empty.insert(global_hotkey::GlobalHotKeyManager::new().map_err(Error::shortcut)?)
            }
        };
        hotkeys.register(hotkey).map_err(Error::shortcut)?;
        let presses = crate::shortcut::channel();
        let heard = presses.subscribe();
        self.shortcuts.insert(hotkey.id(), presses);
        Ok(crate::Presses::new(heard))
    }

    /// Answers the item `id` of the menu bar or of the tray's menu.
    fn chosen(&mut self, id: &tray_icon::menu::MenuId) {
        if id.0 == crate::tray::QUIT || is_menu_quit(id) {
            self.quit();
        } else if let Some(clicked) = self.items.get(id) {
            let _ = clicked.send(());
        }
    }

    /// Hides the window `id` and sends its page to `about:blank`, which ends
    /// the page's streams from the web view's side, so the server has no
    /// open stream to wait for when it is asked to stop. The window goes on
    /// [`Event::Left`], after [`LEAVE`].
    fn leave(&mut self, id: WindowId) {
        let Some(open) = self.windows.get_mut(&id) else {
            return;
        };
        if open.leaving {
            return;
        }
        open.leaving = true;
        open.window.set_visible(false);
        if let Err(error) = open.web_view.load_url("about:blank") {
            tracing::error!(%error, "a closing window could not leave its page");
        }
        let left = self.proxy.clone();
        self.runtime.spawn(async move {
            tokio::time::sleep(LEAVE).await;
            let _ = left.send_event(Event::Left(Some(id)));
        });
    }
}

/// Installs the package an update left for the end of the run. The
/// installer replaces this program and, on Windows, ends the process, so it
/// starts only after the server has stopped.
#[cfg(feature = "updater")]
fn install_after_stop(install: Option<crate::Downloaded>) -> Result<(), Error> {
    install.map_or(Ok(()), crate::Downloaded::install_now)
}

/// Whether `id` is the Quit item of the macOS menu bar.
#[cfg(target_os = "macos")]
fn is_menu_quit(id: &tray_icon::menu::MenuId) -> bool {
    id.0 == crate::menu::QUIT
}

/// Whether `id` is the Quit item of the macOS menu bar, which this system has none of.
#[cfg(not(target_os = "macos"))]
fn is_menu_quit(_id: &tray_icon::menu::MenuId) -> bool {
    false
}

/// Brings a window forward each time a second copy hands its command line over.
async fn raise_on_launch(
    mut launches: tokio::sync::broadcast::Receiver<handoff::Launch>,
    proxy: EventLoopProxy<Event>,
) {
    use tokio::sync::broadcast::error::RecvError;
    loop {
        match launches.recv().await {
            Ok(_) | Err(RecvError::Lagged(_)) => {
                if proxy.send_event(Event::Raise).is_err() {
                    return;
                }
            }
            Err(RecvError::Closed) => return,
        }
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
    #[cfg(target_os = "linux")]
    let router = crate::framing::wrap(router);
    let app = guard::wrap(router, guard);
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
