//! Global shortcuts, which the application hears while another application
//! has focus, as push-to-talk and a show-and-hide key need.

use global_hotkey::hotkey::{Code, HotKey};
use tokio::sync::broadcast;

/// The modifier keys of a [`Shortcut`], joined with [`and`](Self::and).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    bits: u8,
}

impl Modifiers {
    /// No modifier, which only a function key may have.
    pub const NONE: Self = Self { bits: 0 };
    /// The Shift key.
    pub const SHIFT: Self = Self { bits: 1 };
    /// The Control key, on macOS as well.
    pub const CONTROL: Self = Self { bits: 1 << 1 };
    /// The Alt key, which macOS labels Option.
    pub const ALT: Self = Self { bits: 1 << 2 };
    /// The Command key on macOS and the Windows key elsewhere.
    pub const SUPER: Self = Self { bits: 1 << 3 };
    /// Command on macOS and Control elsewhere, the key of each system's own shortcuts.
    #[cfg(target_os = "macos")]
    pub const PRIMARY: Self = Self::SUPER;
    /// Command on macOS and Control elsewhere, the key of each system's own shortcuts.
    #[cfg(not(target_os = "macos"))]
    pub const PRIMARY: Self = Self::CONTROL;

    /// These modifiers and `other` together.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    const fn has(self, other: Self) -> bool {
        self.bits & other.bits == other.bits
    }

    /// The same modifiers, as `global-hotkey` names them.
    fn system(self) -> global_hotkey::hotkey::Modifiers {
        use global_hotkey::hotkey::Modifiers as System;
        [
            (Self::SHIFT, System::SHIFT),
            (Self::CONTROL, System::CONTROL),
            (Self::ALT, System::ALT),
            (Self::SUPER, System::META),
        ]
        .into_iter()
        .filter(|&(ours, _)| self.has(ours))
        .fold(System::empty(), |all, (_, system)| all | system)
    }
}

/// The key of a [`Shortcut`], named by where it is on a US keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[expect(missing_docs, reason = "each variant is the key it names")]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

impl Key {
    const fn is_function(self) -> bool {
        matches!(
            self,
            Self::F1
                | Self::F2
                | Self::F3
                | Self::F4
                | Self::F5
                | Self::F6
                | Self::F7
                | Self::F8
                | Self::F9
                | Self::F10
                | Self::F11
                | Self::F12
        )
    }

    /// The same key, as `global-hotkey` names it.
    fn code(self) -> Code {
        match self {
            Self::A => Code::KeyA,
            Self::B => Code::KeyB,
            Self::C => Code::KeyC,
            Self::D => Code::KeyD,
            Self::E => Code::KeyE,
            Self::F => Code::KeyF,
            Self::G => Code::KeyG,
            Self::H => Code::KeyH,
            Self::I => Code::KeyI,
            Self::J => Code::KeyJ,
            Self::K => Code::KeyK,
            Self::L => Code::KeyL,
            Self::M => Code::KeyM,
            Self::N => Code::KeyN,
            Self::O => Code::KeyO,
            Self::P => Code::KeyP,
            Self::Q => Code::KeyQ,
            Self::R => Code::KeyR,
            Self::S => Code::KeyS,
            Self::T => Code::KeyT,
            Self::U => Code::KeyU,
            Self::V => Code::KeyV,
            Self::W => Code::KeyW,
            Self::X => Code::KeyX,
            Self::Y => Code::KeyY,
            Self::Z => Code::KeyZ,
            Self::Digit0 => Code::Digit0,
            Self::Digit1 => Code::Digit1,
            Self::Digit2 => Code::Digit2,
            Self::Digit3 => Code::Digit3,
            Self::Digit4 => Code::Digit4,
            Self::Digit5 => Code::Digit5,
            Self::Digit6 => Code::Digit6,
            Self::Digit7 => Code::Digit7,
            Self::Digit8 => Code::Digit8,
            Self::Digit9 => Code::Digit9,
            Self::F1 => Code::F1,
            Self::F2 => Code::F2,
            Self::F3 => Code::F3,
            Self::F4 => Code::F4,
            Self::F5 => Code::F5,
            Self::F6 => Code::F6,
            Self::F7 => Code::F7,
            Self::F8 => Code::F8,
            Self::F9 => Code::F9,
            Self::F10 => Code::F10,
            Self::F11 => Code::F11,
            Self::F12 => Code::F12,
            Self::Space => Code::Space,
            Self::Enter => Code::Enter,
            Self::Escape => Code::Escape,
            Self::Tab => Code::Tab,
            Self::Backspace => Code::Backspace,
            Self::ArrowUp => Code::ArrowUp,
            Self::ArrowDown => Code::ArrowDown,
            Self::ArrowLeft => Code::ArrowLeft,
            Self::ArrowRight => Code::ArrowRight,
        }
    }
}

/// A key and its modifiers, for [`Os::shortcut`](crate::Os::shortcut).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Shortcut {
    modifiers: Modifiers,
    key: Key,
}

impl Shortcut {
    /// The shortcut of `key` held with `modifiers`.
    ///
    /// # Panics
    ///
    /// Panics when `modifiers` is [`Modifiers::NONE`] and `key` is not a
    /// function key, since the application would take that key from every
    /// other application. In a `const`, that is a compile error.
    ///
    /// ```compile_fail,E0080
    /// use hypersig_desktop::{Key, Modifiers, Shortcut};
    ///
    /// const TALK: Shortcut = Shortcut::new(Modifiers::NONE, Key::Space);
    /// # let _ = TALK;
    /// ```
    #[must_use]
    pub const fn new(modifiers: Modifiers, key: Key) -> Self {
        assert!(
            modifiers.bits != 0 || key.is_function(),
            "a global shortcut holds a modifier, unless its key is a function key, since it takes the key from every other application"
        );
        Self { modifiers, key }
    }

    /// The same shortcut, as `global-hotkey` registers it.
    pub(crate) fn hotkey(self) -> HotKey {
        HotKey::new(Some(self.modifiers.system()), self.key.code())
    }
}

/// Whether a [`Shortcut`] went down or came up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    clippy::exhaustive_enums,
    reason = "a key goes down or comes up, and there is no third state"
)]
pub enum Press {
    /// The shortcut's keys went down.
    Pressed,
    /// The shortcut's key came up.
    Released,
}

impl From<global_hotkey::HotKeyState> for Press {
    fn from(state: global_hotkey::HotKeyState) -> Self {
        match state {
            global_hotkey::HotKeyState::Pressed => Self::Pressed,
            global_hotkey::HotKeyState::Released => Self::Released,
        }
    }
}

/// The presses and releases of a [`Shortcut`], from [`Os::shortcut`](crate::Os::shortcut).
#[derive(Debug)]
pub struct Presses {
    receiver: broadcast::Receiver<Press>,
}

impl Presses {
    pub(crate) fn new(receiver: broadcast::Receiver<Press>) -> Self {
        Self { receiver }
    }

    /// Waits for the next press or release, and returns `None` once the
    /// application has stopped.
    pub async fn next(&mut self) -> Option<Press> {
        loop {
            match self.receiver.recv().await {
                Ok(press) => return Some(press),
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    }
}

/// Where a shortcut's presses go, for every [`Presses`] that waits on them.
pub(crate) fn channel() -> broadcast::Sender<Press> {
    broadcast::channel(16).0
}

#[cfg(test)]
mod tests {
    use super::{Key, Modifiers, Shortcut};

    #[test]
    fn a_shortcut_registers_its_modifiers_and_key() {
        let talk = Shortcut::new(Modifiers::CONTROL.and(Modifiers::SHIFT), Key::Space);
        let hotkey = talk.hotkey();
        assert_eq!(hotkey.key, global_hotkey::hotkey::Code::Space);
        assert_eq!(
            hotkey.mods,
            global_hotkey::hotkey::Modifiers::CONTROL | global_hotkey::hotkey::Modifiers::SHIFT
        );
        assert_ne!(
            talk.hotkey().id(),
            Shortcut::new(Modifiers::ALT, Key::Space).hotkey().id()
        );
        let _ = Shortcut::new(Modifiers::NONE, Key::F9);
    }
}
