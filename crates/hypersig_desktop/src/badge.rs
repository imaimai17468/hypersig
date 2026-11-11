//! The count on the application's icon: the Dock on macOS, the launcher on
//! Linux, and the taskbar button of each window on Windows.

use tao::event_loop::EventLoopWindowTarget;
use tao::window::Window;

use crate::run::Event;

/// Shows `count` on the application's icon, or clears it at 0.
///
/// macOS writes the number on the Dock icon, whichever window it is set
/// through. Linux sets it on the launcher entry of the desktop file named
/// after the program, which Unity's launcher API reads and GNOME without an
/// extension does not. Windows has no number on a taskbar button, so each
/// window gets a red dot while the count is above 0.
#[cfg_attr(
    target_os = "linux",
    expect(
        clippy::needless_pass_by_value,
        reason = "Linux sets the count on the launcher entry, and no window takes it"
    )
)]
pub(crate) fn show<'a>(
    count: u32,
    windows: impl IntoIterator<Item = &'a Window>,
    target: &EventLoopWindowTarget<Event>,
) {
    #[cfg(target_os = "macos")]
    {
        use tao::platform::macos::WindowExtMacOS as _;
        let _ = target;
        if let Some(window) = windows.into_iter().next() {
            window.set_badge_label((count > 0).then(|| count.to_string()));
        }
    }
    #[cfg(target_os = "linux")]
    {
        use tao::platform::unix::EventLoopWindowTargetExtUnix as _;
        let _ = windows;
        target.set_badge_count((count > 0).then_some(i64::from(count)), desktop_file());
    }
    #[cfg(windows)]
    {
        let _ = target;
        for window in windows {
            on_taskbar(window, count);
        }
    }
}

/// Puts the dot on `window`'s taskbar button while `count` is above 0, for
/// a window opened while the count is shown.
#[cfg(windows)]
pub(crate) fn on_taskbar(window: &Window, count: u32) {
    use tao::platform::windows::WindowExtWindows as _;
    let dot = if count > 0 { dot() } else { None };
    window.set_overlay_icon(dot.as_ref());
}

/// A red dot, 16 pixels across, on a transparent ground.
#[cfg(windows)]
fn dot() -> Option<tao::window::Icon> {
    const SIZE: u32 = 16;
    let mut rgba = Vec::new();
    for y in 0..SIZE {
        for x in 0..SIZE {
            let (dx, dy) = (f64::from(x) - 7.5, f64::from(y) - 7.5);
            let inside = dx * dx + dy * dy <= 7.5 * 7.5;
            rgba.extend_from_slice(if inside {
                &[0xe5, 0x39, 0x35, 0xff]
            } else {
                &[0, 0, 0, 0]
            });
        }
    }
    tao::window::Icon::from_rgba(rgba, SIZE, SIZE).ok()
}

/// `<program>.desktop`, the name cargo-packager gives the desktop file of
/// a Debian package and an `AppImage`.
#[cfg(target_os = "linux")]
fn desktop_file() -> Option<String> {
    let program = std::env::current_exe().ok()?;
    Some(format!("{}.desktop", program.file_stem()?.to_str()?))
}
