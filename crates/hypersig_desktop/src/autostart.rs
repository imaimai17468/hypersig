//! Starting the application when the person logs in: a launch agent on
//! macOS, the `Run` key of the registry on Windows, and an autostart entry
//! on Linux, which `auto-launch` writes.

use std::path::PathBuf;

use auto_launch::{AutoLaunch, AutoLaunchBuilder};

use crate::error::Error;

/// The entry that starts the program of the application named `id`.
pub(crate) fn entry(id: &str) -> Result<AutoLaunch, Error> {
    let program = program()?;
    AutoLaunchBuilder::new()
        .set_app_name(id)
        .set_app_path(&program.to_string_lossy())
        .build()
        .map_err(Error::autostart)
}

/// The program to start: on Linux the `AppImage` the person installed,
/// whose path its runtime puts in `APPIMAGE`, since the running binary
/// is a copy mounted for this run, and elsewhere the running binary.
fn program() -> Result<PathBuf, Error> {
    #[cfg(target_os = "linux")]
    if let Some(image) = std::env::var_os("APPIMAGE") {
        return Ok(PathBuf::from(image));
    }
    std::env::current_exe().map_err(Error::autostart)
}

/// Runs `change` on the entry of `id` on a blocking thread, since the
/// system's calls may wait for a program they run.
pub(crate) async fn with_entry<T: Send + 'static>(
    id: &'static str,
    change: impl FnOnce(&AutoLaunch) -> Result<T, auto_launch::Error> + Send + 'static,
) -> Result<T, Error> {
    tokio::task::spawn_blocking(move || change(&entry(id)?).map_err(Error::autostart))
        .await
        .map_err(Error::autostart)?
}
