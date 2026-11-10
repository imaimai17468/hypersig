//! The system clipboard, kept by one thread for the whole run.
//!
//! On Linux, text written to the clipboard is served by the process that
//! wrote it, for as long as its `arboard::Clipboard` lives, so the clipboard
//! is made once and kept by a thread of its own. Each request reaches that
//! thread over a channel and is answered on a channel of its own.

use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};
use std::thread;

use tokio::sync::oneshot;

use crate::error::Error;

/// What the clipboard thread is asked to do.
enum Request {
    Read(oneshot::Sender<Result<Option<String>, Error>>),
    Write(String, oneshot::Sender<Result<(), Error>>),
}

/// The channel to the clipboard thread, which starts on first use.
fn thread() -> Sender<Request> {
    static THREAD: OnceLock<Sender<Request>> = OnceLock::new();
    THREAD
        .get_or_init(|| {
            let (requests, received) = mpsc::channel::<Request>();
            thread::spawn(move || {
                let mut clipboard = arboard::Clipboard::new().map_err(Error::clipboard);
                for request in received {
                    match request {
                        Request::Read(answer) => {
                            let text = match &mut clipboard {
                                Ok(clipboard) => match clipboard.get_text() {
                                    Ok(text) => Ok(Some(text)),
                                    Err(arboard::Error::ContentNotAvailable) => Ok(None),
                                    Err(error) => Err(Error::clipboard(error)),
                                },
                                Err(_) => {
                                    Err(Error::clipboard("the clipboard could not be opened"))
                                }
                            };
                            let _ = answer.send(text);
                        }
                        Request::Write(text, answer) => {
                            let written = match &mut clipboard {
                                Ok(clipboard) => clipboard.set_text(text).map_err(Error::clipboard),
                                Err(_) => {
                                    Err(Error::clipboard("the clipboard could not be opened"))
                                }
                            };
                            let _ = answer.send(written);
                        }
                    }
                }
            });
            requests
        })
        .clone()
}

/// The text on the clipboard, or `None` when it holds no text.
pub(crate) async fn read() -> Result<Option<String>, Error> {
    let (answer, answered) = oneshot::channel();
    thread()
        .send(Request::Read(answer))
        .map_err(|_gone| Error::clipboard("the clipboard thread has stopped"))?;
    answered
        .await
        .map_err(|_gone| Error::clipboard("the clipboard thread has stopped"))?
}

/// Puts `text` on the clipboard.
pub(crate) async fn write(text: String) -> Result<(), Error> {
    let (answer, answered) = oneshot::channel();
    thread()
        .send(Request::Write(text, answer))
        .map_err(|_gone| Error::clipboard("the clipboard thread has stopped"))?;
    answered
        .await
        .map_err(|_gone| Error::clipboard("the clipboard thread has stopped"))?
}
