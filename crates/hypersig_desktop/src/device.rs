//! The capture devices a page may use through `getUserMedia` and
//! `getDisplayMedia`, which the
//! application names with [`App::allow`](crate::App::allow).

use wry::{PermissionKind, PermissionResponse};

/// A capture device the pages may use, for [`App::allow`](crate::App::allow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Device {
    /// The microphone, for `getUserMedia({ audio: true })`.
    Microphone,
    /// The camera, for `getUserMedia({ video: true })`.
    Camera,
    /// The screen, for `getDisplayMedia`, as sharing it in a call does.
    ///
    /// Only `WebKitGTK`, on Linux, asks the application about it, and shares
    /// the screen from a click of the person's alone. `WebView2`, on Windows,
    /// answers `getDisplayMedia` on its own, whether or not the application
    /// names the screen. `WKWebView`, on macOS, has no `getDisplayMedia`, so
    /// a page there cannot share the screen.
    Screen,
}

/// The devices an application allows, none by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Devices {
    microphone: bool,
    camera: bool,
    screen: bool,
}

impl Devices {
    /// No device.
    pub(crate) const NONE: Self = Self {
        microphone: false,
        camera: false,
        screen: false,
    };

    /// These devices and `device`.
    pub(crate) const fn with(mut self, device: Device) -> Self {
        match device {
            Device::Microphone => self.microphone = true,
            Device::Camera => self.camera = true,
            Device::Screen => self.screen = true,
        }
        self
    }

    /// Whether any device is allowed.
    #[cfg_attr(
        all(target_os = "macos", not(test)),
        expect(
            dead_code,
            reason = "only WebKitGTK and a smoke run's WebView2 read it"
        )
    )]
    pub(crate) const fn any(self) -> bool {
        self.microphone || self.camera || self.screen
    }

    /// The web view's answer to a page asking for `kind`. A device the
    /// application did not allow is refused without asking the person, and
    /// a request for anything else gets the web view's own answer.
    pub(crate) fn answer(self, kind: PermissionKind) -> PermissionResponse {
        let allowed = match kind {
            PermissionKind::Microphone => self.microphone,
            PermissionKind::Camera => self.camera,
            PermissionKind::DisplayCapture => self.screen,
            _ => return PermissionResponse::Default,
        };
        if allowed {
            PermissionResponse::Allow
        } else {
            PermissionResponse::Deny
        }
    }
}

#[cfg(test)]
mod tests {
    use wry::{PermissionKind, PermissionResponse};

    use super::{Device, Devices};

    #[test]
    fn only_the_devices_allowed_are_granted() {
        let devices = Devices::NONE.with(Device::Microphone);
        assert!(devices.any());
        assert!(!Devices::NONE.any());
        assert_eq!(
            devices.answer(PermissionKind::Microphone),
            PermissionResponse::Allow
        );
        assert_eq!(
            devices.answer(PermissionKind::Camera),
            PermissionResponse::Deny
        );
        assert_eq!(
            devices.answer(PermissionKind::DisplayCapture),
            PermissionResponse::Deny
        );
        assert_eq!(
            Devices::NONE
                .with(Device::Screen)
                .answer(PermissionKind::DisplayCapture),
            PermissionResponse::Allow
        );
        assert_eq!(
            devices.answer(PermissionKind::Geolocation),
            PermissionResponse::Default
        );
    }
}
