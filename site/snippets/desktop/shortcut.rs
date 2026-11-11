# use hypersig_desktop::testing::FakeOs;
use hypersig_desktop::{Key, Modifiers, Os, Press, Shortcut};
use tokio::sync::watch;

/// Command+Shift+Space on macOS, and Control+Shift+Space elsewhere.
const TALK: Shortcut = Shortcut::new(Modifiers::PRIMARY.and(Modifiers::SHIFT), Key::Space);

/// Keeps `held` true while the person holds the push-to-talk key.
async fn push_to_talk(os: &Os, held: watch::Sender<bool>) -> Result<(), hypersig_desktop::Error> {
    let mut presses = os.shortcut(TALK).await?;
    tokio::spawn(async move {
        while let Some(press) = presses.next().await {
            held.send_replace(press == Press::Pressed);
        }
    });
    Ok(())
}
# fn main() {
# tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# let (held, mut talking) = watch::channel(false);
# push_to_talk(&fake.os(), held).await.unwrap();
# fake.press_shortcut(TALK, Press::Pressed);
# talking.changed().await.unwrap();
# assert!(*talking.borrow());
# });
# }
