//! The capture devices a page may use through `getUserMedia`, which the
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
}

/// The devices an application allows, none by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Devices {
    microphone: bool,
    camera: bool,
}

impl Devices {
    /// No device.
    pub(crate) const NONE: Self = Self {
        microphone: false,
        camera: false,
    };

    /// These devices and `device`.
    pub(crate) const fn with(mut self, device: Device) -> Self {
        match device {
            Device::Microphone => self.microphone = true,
            Device::Camera => self.camera = true,
        }
        self
    }

    /// Whether any device is allowed.
    #[cfg_attr(
        not(any(target_os = "linux", test)),
        expect(dead_code, reason = "only WebKitGTK turns media streams on")
    )]
    pub(crate) const fn any(self) -> bool {
        self.microphone || self.camera
    }

    /// The web view's answer to a page asking for `kind`. A device the
    /// application did not allow is refused without asking the person, and
    /// a request for anything else gets the web view's own answer.
    pub(crate) fn answer(self, kind: PermissionKind) -> PermissionResponse {
        let allowed = match kind {
            PermissionKind::Microphone => self.microphone,
            PermissionKind::Camera => self.camera,
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
            devices.answer(PermissionKind::Geolocation),
            PermissionResponse::Default
        );
    }
}
