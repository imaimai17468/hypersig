//! Smoke runs, which start the application, check its first page, and close it.
//!
//! With `HYPERSIG_DESKTOP_SMOKE` set, the run writes how long after
//! [`App::run`](crate::App::run) the first page was shown, checks that it is
//! one hypersig rendered, and closes the application. With
//! `HYPERSIG_DESKTOP_SMOKE_SCRIPT` set to the path of a JavaScript file, it
//! then runs that file in the first page, and in the page of each window
//! opened after it, and waits for the first page to set
//! `window.hypersigSmoke` to `"pass"`, or to anything else to fail.

use std::env;
use std::fs;
use std::time::{Duration, Instant};

use tao::event_loop::EventLoopProxy;
use tokio::runtime::Runtime;
use wry::WebView;

use crate::error::Error;
use crate::run::Event;

const SMOKE: &str = "HYPERSIG_DESKTOP_SMOKE";
const SCRIPT: &str = "HYPERSIG_DESKTOP_SMOKE_SCRIPT";

/// Whether the page is one hypersig rendered, which a refused request is not.
const RENDERED: &str = "document.body !== null && document.body.dataset.hsSignals !== undefined";

/// What the script has set, or `""`.
const VERDICT: &str = "String(window.hypersigSmoke ?? '')";

/// How often the run asks the page for the script's verdict.
const POLL: Duration = Duration::from_millis(200);

/// How long the script is given to set its verdict.
const SCRIPT_TIME: Duration = Duration::from_secs(20);

/// A smoke run, and how far it has come.
#[derive(Debug)]
pub(crate) struct Smoke {
    started: Instant,
    script: Option<String>,
    stage: Stage,
}

#[derive(Debug)]
enum Stage {
    /// Waiting for the first page's answer to [`RENDERED`].
    Rendering,
    /// Waiting until `deadline` for the script's verdict.
    Scripting { deadline: Instant },
}

impl Smoke {
    /// The smoke run the environment asks for, if any.
    ///
    /// # Errors
    ///
    /// Fails when the script named in `HYPERSIG_DESKTOP_SMOKE_SCRIPT` cannot be read.
    pub(crate) fn from_env(started: Instant) -> Result<Option<Self>, Error> {
        let script = env::var_os(SCRIPT).map(fs::read_to_string).transpose()?;
        if script.is_none() && env::var_os(SMOKE).is_none() {
            return Ok(None);
        }
        Ok(Some(Self {
            started,
            script,
            stage: Stage::Rendering,
        }))
    }

    /// Takes the next step, with `answer` being the page's answer to the
    /// last question, or `None` when it is time to ask. The answer to a
    /// question arrives as [`Event::Smoke`]. Returns whether the run is done.
    ///
    /// # Errors
    ///
    /// Fails when the first page is not hypersig's, when the script sets
    /// anything but `"pass"` or sets nothing in time, and when the web view
    /// cannot run a script.
    pub(crate) fn step(
        &mut self,
        answer: Option<&str>,
        web_view: &WebView,
        proxy: &EventLoopProxy<Event>,
        runtime: &Runtime,
    ) -> Result<bool, Error> {
        match (&self.stage, answer) {
            // The time is written when the window is shown, as other
            // frameworks' measurements write it, and the check follows; a
            // failed check still ends the run with an error.
            (Stage::Rendering, None) => {
                report(&format!(
                    "first page shown {} ms after run",
                    self.started.elapsed().as_millis()
                ));
                ask(web_view, RENDERED, proxy)?;
            }
            (Stage::Rendering, Some(answer)) => {
                if answer != "true" {
                    return Err(Error::smoke(
                        "the first page is not one hypersig rendered, as when the server refused the window",
                    ));
                }
                let Some(script) = &self.script else {
                    return Ok(true);
                };
                web_view.evaluate_script(script).map_err(Error::window)?;
                self.stage = Stage::Scripting {
                    deadline: Instant::now() + SCRIPT_TIME,
                };
                later(proxy, runtime);
            }
            (Stage::Scripting { .. }, None) => ask(web_view, VERDICT, proxy)?,
            (Stage::Scripting { deadline }, Some(answer)) => {
                // The web view answers with the value as JSON, here a string.
                let verdict = answer.trim_matches('"');
                if verdict == "pass" {
                    report("smoke script passed");
                    return Ok(true);
                }
                if !verdict.is_empty() {
                    return Err(Error::smoke(format!("the script set {verdict}")));
                }
                if Instant::now() > *deadline {
                    return Err(Error::smoke(format!(
                        "the script set no verdict in {} seconds",
                        SCRIPT_TIME.as_secs()
                    )));
                }
                later(proxy, runtime);
            }
        }
        Ok(false)
    }
}

impl Smoke {
    /// Runs the script in the page of a window other than the first, once
    /// the first page runs it.
    ///
    /// # Errors
    ///
    /// Fails when the web view cannot run a script.
    pub(crate) fn other_window(&self, web_view: &WebView) -> Result<(), Error> {
        match (&self.stage, &self.script) {
            (Stage::Scripting { .. }, Some(script)) => {
                web_view.evaluate_script(script).map_err(Error::window)
            }
            _ => Ok(()),
        }
    }
}

/// Runs `question` in the page, whose answer arrives as [`Event::Smoke`].
fn ask(web_view: &WebView, question: &str, proxy: &EventLoopProxy<Event>) -> Result<(), Error> {
    let answers = proxy.clone();
    web_view
        .evaluate_script_with_callback(question, move |answer| {
            let _ = answers.send_event(Event::Smoke(Some(answer)));
        })
        .map_err(Error::window)
}

/// Asks again after [`POLL`].
fn later(proxy: &EventLoopProxy<Event>, runtime: &Runtime) {
    let again = proxy.clone();
    runtime.spawn(async move {
        tokio::time::sleep(POLL).await;
        let _ = again.send_event(Event::Smoke(None));
    });
}

#[expect(
    clippy::print_stdout,
    reason = "a smoke run is read from standard output by whatever started the application"
)]
fn report(what: &str) {
    println!("hypersig_desktop: {what}");
}
